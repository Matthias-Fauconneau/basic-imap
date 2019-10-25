extern crate imap;
extern crate native_tls;

extern crate nix;
extern crate dbus;
mod accounts;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", fetch_inbox_top( &email()?, &password()? )? );
    Ok(())
}

fn email() -> Result<String, Box<dyn std::error::Error>> {
    // dbus-codegen-rust -s -g -m None -i org.freedesktop -d org.freedesktop.Accounts -p /org/freedesktop/Accounts/User1000 > src/accounts.rs 
    use accounts::AccountsUser;
    Ok(dbus::blocking::Connection::new_system()?.with_proxy("org.freedesktop.Accounts", format!("/org/freedesktop/Accounts/User{}",nix::unistd::getuid()), core::time::Duration::new(1,0)).get_email()?)
}

fn password() -> Result<String, Box<dyn std::error::Error>> {
    use std::io::BufRead;
    Ok(std::process::Command::new("kwallet-query").args(&["kdewallet", "-f","imap", "-r","akonadi_imap_resource_0rc"]).output()?.stdout.lines().next().unwrap()?)
}

fn fetch_inbox_top(email : &str, password : &str) -> Result<String, Box<dyn std::error::Error>> {
    let domain = "imap.gmail.com";
    let tls = native_tls::TlsConnector::builder().build().unwrap();

    // we pass in the domain twice to check that the server's TLS
    // certificate is valid for the domain we're connecting to.
    let client = imap::connect((domain, 993), domain, &tls).unwrap();

    // the client we have here is unauthenticated.
    // to do anything useful with the e-mails, we need to log in
    let mut imap_session = client
        .login(email, password)
        .map_err(|e| e.0)?;

    // we want to fetch the first email in the INBOX mailbox
    imap_session.select("INBOX")?;

    // fetch message number 1 in this mailbox, along with its RFC822 field.
    // RFC 822 dictates the format of the body of e-mails
    let messages = imap_session.fetch("1", "RFC822")?;
    let message = if let Some(m) = messages.iter().next() {
        m
    } else {
        return Ok(String::new());
    };

    // extract the message's body
    let body = message.body().expect("message did not have a body!");
    let body = std::str::from_utf8(body)
        .expect("message was not valid utf-8")
        .to_string();

    // be nice to the server and log out
    imap_session.logout()?;

    Ok(body)
}
