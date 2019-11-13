#!/usr/bin/fish -c clear && cargo +nightly run
#![feature(backtrace)]
#![feature(never_type)]

#[derive(Debug)] struct NoneError;
impl std::fmt::Display for NoneError { fn fmt(&self, f : &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { (self as &dyn std::fmt::Debug).fmt(f) } }
impl std::error::Error for NoneError {}

extern crate anyhow;
use anyhow::{Result, Context};

extern crate dbus;
mod accounts;
fn email(domain : &str) -> Result<String> {
    // dbus-codegen-rust -s -g -m None -i org.freedesktop -d org.freedesktop.Accounts -p /org/freedesktop/Accounts/User1000 > src/accounts.rs
    use accounts::AccountsUser;
    let path = format!("/org/freedesktop/Accounts/User{}",nix::unistd::getuid());
    dbus::blocking::Connection::new_system()?.with_proxy("org.freedesktop.Accounts", path, core::time::Duration::new(1,0)).get_email()
        .or_else( |_| Ok(format!("{}@{}", std::env::var("LOGNAME")?, domain)) )
}

fn password() -> Result<String> {
    use std::io::BufRead;
    Ok(std::process::Command::new("kwallet-query").args(&["kdewallet","-f","imap","-r","akonadi_imap_resource_0rc"]).output()?.stdout.lines().next().ok_or(NoneError)??)
}

trait StripEnd {
    fn strip_end<'a, Patterns : IntoIterator<Item = &'a Self>>(&self, patterns : Patterns) -> &Self where Self : 'a;
}
impl StripEnd for str {
    fn strip_end<'a, Patterns : IntoIterator<Item = &'a Self>> (&self, patterns : Patterns) -> &Self where Self : 'a {
        for pattern in patterns {
            if self.ends_with(&pattern) {
                return &self[0..self.len()-pattern.len()];
            }
        }
        self
    }
}

extern crate mailparse;
fn main() -> Result<!> {
    let tls = native_tls::TlsConnector::builder().build()?;
    let domain = "gmail.com";
    let username = email(domain)?;
    let password = password()?;
    let imap_domain = format!("imap.{}", domain);
    let client = imap::connect((&imap_domain as &str, 993), &imap_domain, &tls)?;
    let mut session = client.login(username, password).map_err(|(e, _)| e)?; // login requires to explicitly drop client connection on error
    session.select("INBOX")?;

    let mut last_notified = 0;
    loop {
        let mut uids = session.uid_search("UNSEEN 1:*")?;
        let num_unseen = uids.len();
        if uids.iter().all(|&uid| uid <= last_notified) { uids.clear(); } // there are no messages we haven't already notified about
        last_notified = std::cmp::max(last_notified, uids.iter().cloned().max().unwrap_or(0));

        let mut subjects = std::collections::BTreeMap::new();
        if !uids.is_empty() {
            let uids: Vec<_> = uids.into_iter().map(|v: u32| format!("{}", v)).collect();
            for msg in session.uid_fetch(&uids.join(","), "RFC822.HEADER")?.iter().flat_map(|msg| msg.header()) {
                let (headers, _) = mailparse::parse_headers(msg)?;
                use mailparse::MailHeaderMap;
                let subject = headers.get_first_value("Subject")?.unwrap_or_else(||"<no subject>".into());
                println!("{}", subject);
                let date = headers.get_first_value("Date")?.ok_or(NoneError)?;
                let subs = ["UTC"," CST"].iter().map(|s| { format!(" ({})",s) }).collect::<Vec<_>>();
                let date = date.strip_end(subs.iter().map(|x|&**x));
                let date = chrono::DateTime::parse_from_str(&date, "%a, %e %b %Y %H:%M:%S %z").with_context(||date.to_string())?.with_timezone(&chrono::Local);
                subjects.insert(date, subject);
            }

            if !subjects.is_empty() {
                if let Err(e) = notify_rust::Notification::new().summary( &format!("{}",num_unseen) )
                                                                                        .body( &subjects.values().rev().map(|s| &**s).collect::<Vec<&str>>().join("\n") )
                                                                                        .icon("notification-message-email")
                                                                                        .hint(notify_rust::NotificationHint::Category("email.arrived".into()))
                                                                                        .show()
                                    { println!("{:?}", e); }
            }

            session.idle()?.wait_keepalive()?;
        }
    }
}
