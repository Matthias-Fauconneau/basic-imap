pub struct Error(Box<dyn std::fmt::Debug>);
impl<T: std::fmt::Debug + 'static> From<T> for Error { fn from(error : T) -> Self { Error(Box::new(error)) } }
type Result<T> = std::result::Result<T, Error>;
fn from<T>(result : Result<T>) -> std::result::Result<T, Box<dyn std::fmt::Debug>> { match result { Ok(t) => Ok(t), Err(e) => Err(e.0) } }
fn main() -> std::result::Result<(), Box<dyn std::fmt::Debug>> { from(run()) }

extern crate nix;
extern crate dbus;
mod accounts;
fn email() -> Result<String> {
    // dbus-codegen-rust -s -g -m None -i org.freedesktop -d org.freedesktop.Accounts -p /org/freedesktop/Accounts/User1000 > src/accounts.rs
    use accounts::AccountsUser;
    let path = format!("/org/freedesktop/Accounts/User{}",nix::unistd::getuid());
    Ok(dbus::blocking::Connection::new_system()?.with_proxy("org.freedesktop.Accounts", path, core::time::Duration::new(1,0)).get_email()?)
}

fn password() -> Result<String> {
    use std::io::BufRead;
    Ok(std::process::Command::new("kwallet-query").args(&["kdewallet","-f","imap","-r","akonadi_imap_resource_0rc"]).output()?.stdout.lines().next()??)
}

extern crate native_tls;
extern crate imap;
extern crate mailparse;
extern crate chrono;
extern crate notify_rust;
fn run() -> Result<()> {
    let tls = native_tls::TlsConnector::builder().build()?;
    let domain = "imap.gmail.com";
    let username = email()?;
    let password = password()?;
    let client = imap::connect((domain, 993), &domain, &tls)?;
    let mut session = client.login(username, password).map_err(|(e, _)| e)?; // login requires to explicitly drop client connection on error
    //if !session.capabilities().iter().any(|&capability| capability == "IDLE") { return Err(session.capabilities().iter().cloned().collect()); } // Assumes IDLE
    session.select("INBOX")?;

    // app.set_icon(connect);

  // Keep track of all the e-mails we have already notified about
    let mut last_notified = 0;
    //let mut notification = None::<notify_rust::NotificationHandle>;

    loop {
        // check current state of inbox
        let mut uids = session.uid_search("UNSEEN 1:*")?;
        let num_unseen = uids.len();
        if uids.iter().all(|&uid| uid <= last_notified) { uids.clear(); } // there are no messages we haven't already notified about
        last_notified = std::cmp::max(last_notified, uids.iter().cloned().max().unwrap_or(0));

        let mut subjects = std::collections::BTreeMap::new();
        if !uids.is_empty() {
            let uids: Vec<_> = uids.into_iter().map(|v: u32| format!("{}", v)).collect();
            for msg in session.uid_fetch(&uids.join(","), "RFC822.HEADER")?.iter() {
                let msg = msg.header();
                if msg.is_none() { continue; }
  
                let (headers, _) = mailparse::parse_headers(msg?)?;
                use mailparse::MailHeaderMap;

                let subject = headers.get_first_value("Subject")?/*gratuitous Result*/.unwrap_or("<no subject>".into());
                println!("{}", subject);
                let date = chrono::DateTime::parse_from_rfc2822( &headers.get_first_value("Date")?? )?.with_timezone(&chrono::Local);
                subjects.insert(date, subject);
            }

            if !subjects.is_empty() {
                // List in reverse chronological order
                notify_rust::Notification::new().summary( &format!("{}",num_unseen) )
                                                                      .body( &subjects.values().rev().map(|s| &**s).collect::<Vec<&str>>().join("\n") )
                                                                      .icon("notification-message-email")
                                                                      .hint(notify_rust::NotificationHint::Category("email.arrived".into()))
                                                                      .show()?;
            }
            
            // IDLE until we see changes
            session.idle()?.wait_keepalive()?;
        }
    }
    // // app.set_icon( mail-unread | mail-unread-new )
    //session.logout()?; // FIXME: unreachable after infinite loop should be in session destructor/drop to logout on break/error
    //Ok(())
}
