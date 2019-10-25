// This code was autogenerated with dbus-codegen-rust, see https://github.com/diwic/dbus-rs

use dbus as dbus;
use dbus::arg;
use dbus::blocking;

pub trait DBusProperties {
    fn get<R0: for<'b> arg::Get<'b>>(&self, interface_name: &str, property_name: &str) -> Result<arg::Variant<R0>, dbus::Error>;
    fn get_all(&self, interface_name: &str) -> Result<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, dbus::Error>;
    fn set<I2: arg::Arg + arg::Append>(&self, interface_name: &str, property_name: &str, value: arg::Variant<I2>) -> Result<(), dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target=blocking::Connection>> DBusProperties for blocking::Proxy<'a, C> {

    fn get<R0: for<'b> arg::Get<'b>>(&self, interface_name: &str, property_name: &str) -> Result<arg::Variant<R0>, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "Get", (interface_name, property_name, ))
            .and_then(|r: (arg::Variant<R0>,)| Ok(r.0))
    }

    fn get_all(&self, interface_name: &str) -> Result<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "GetAll", (interface_name, ))
            .and_then(|r: (::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,)| Ok(r.0))
    }

    fn set<I2: arg::Arg + arg::Append>(&self, interface_name: &str, property_name: &str, value: arg::Variant<I2>) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "Set", (interface_name, property_name, value, ))
    }
}

#[derive(Debug)]
pub struct DBusPropertiesPropertiesChanged {
    pub interface_name: String,
    pub changed_properties: ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
    pub invalidated_properties: Vec<String>,
}

impl arg::AppendAll for DBusPropertiesPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.interface_name, i);
        arg::RefArg::append(&self.changed_properties, i);
        arg::RefArg::append(&self.invalidated_properties, i);
    }
}

impl arg::ReadAll for DBusPropertiesPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(DBusPropertiesPropertiesChanged {
            interface_name: i.read()?,
            changed_properties: i.read()?,
            invalidated_properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for DBusPropertiesPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
}

pub trait DBusIntrospectable {
    fn introspect(&self) -> Result<String, dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target=blocking::Connection>> DBusIntrospectable for blocking::Proxy<'a, C> {

    fn introspect(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
            .and_then(|r: (String,)| Ok(r.0))
    }
}

pub trait DBusPeer {
    fn ping(&self) -> Result<(), dbus::Error>;
    fn get_machine_id(&self) -> Result<String, dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target=blocking::Connection>> DBusPeer for blocking::Proxy<'a, C> {

    fn ping(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.DBus.Peer", "Ping", ())
    }

    fn get_machine_id(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Peer", "GetMachineId", ())
            .and_then(|r: (String,)| Ok(r.0))
    }
}

pub trait AccountsUser {
    fn set_user_name(&self, name: &str) -> Result<(), dbus::Error>;
    fn set_real_name(&self, name: &str) -> Result<(), dbus::Error>;
    fn set_email(&self, email: &str) -> Result<(), dbus::Error>;
    fn set_language(&self, language: &str) -> Result<(), dbus::Error>;
    fn set_formats_locale(&self, formats_locale: &str) -> Result<(), dbus::Error>;
    fn set_input_sources(&self, sources: Vec<::std::collections::HashMap<&str, &str>>) -> Result<(), dbus::Error>;
    fn set_xsession(&self, x_session: &str) -> Result<(), dbus::Error>;
    fn set_session(&self, session: &str) -> Result<(), dbus::Error>;
    fn set_session_type(&self, session_type: &str) -> Result<(), dbus::Error>;
    fn set_location(&self, location: &str) -> Result<(), dbus::Error>;
    fn set_home_directory(&self, homedir: &str) -> Result<(), dbus::Error>;
    fn set_shell(&self, shell: &str) -> Result<(), dbus::Error>;
    fn set_xhas_messages(&self, has_messages: bool) -> Result<(), dbus::Error>;
    fn set_xkeyboard_layouts(&self, layouts: Vec<&str>) -> Result<(), dbus::Error>;
    fn set_background_file(&self, filename: &str) -> Result<(), dbus::Error>;
    fn set_icon_file(&self, filename: &str) -> Result<(), dbus::Error>;
    fn set_locked(&self, locked: bool) -> Result<(), dbus::Error>;
    fn set_account_type(&self, account_type: i32) -> Result<(), dbus::Error>;
    fn set_password_mode(&self, mode: i32) -> Result<(), dbus::Error>;
    fn set_password(&self, password: &str, hint: &str) -> Result<(), dbus::Error>;
    fn set_password_hint(&self, hint: &str) -> Result<(), dbus::Error>;
    fn set_automatic_login(&self, enabled: bool) -> Result<(), dbus::Error>;
    fn get_password_expiration_policy(&self) -> Result<(i64, i64, i64, i64, i64, i64), dbus::Error>;
    fn get_uid(&self) -> Result<u64, dbus::Error>;
    fn get_user_name(&self) -> Result<String, dbus::Error>;
    fn get_real_name(&self) -> Result<String, dbus::Error>;
    fn get_account_type(&self) -> Result<i32, dbus::Error>;
    fn get_home_directory(&self) -> Result<String, dbus::Error>;
    fn get_shell(&self) -> Result<String, dbus::Error>;
    fn get_email(&self) -> Result<String, dbus::Error>;
    fn get_language(&self) -> Result<String, dbus::Error>;
    fn get_session(&self) -> Result<String, dbus::Error>;
    fn get_session_type(&self) -> Result<String, dbus::Error>;
    fn get_formats_locale(&self) -> Result<String, dbus::Error>;
    fn get_input_sources(&self) -> Result<Vec<::std::collections::HashMap<String, String>>, dbus::Error>;
    fn get_xsession(&self) -> Result<String, dbus::Error>;
    fn get_location(&self) -> Result<String, dbus::Error>;
    fn get_login_frequency(&self) -> Result<u64, dbus::Error>;
    fn get_login_time(&self) -> Result<i64, dbus::Error>;
    fn get_login_history(&self) -> Result<Vec<(i64, i64, ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>)>, dbus::Error>;
    fn get_xhas_messages(&self) -> Result<bool, dbus::Error>;
    fn get_xkeyboard_layouts(&self) -> Result<Vec<String>, dbus::Error>;
    fn get_background_file(&self) -> Result<String, dbus::Error>;
    fn get_icon_file(&self) -> Result<String, dbus::Error>;
    fn get_saved(&self) -> Result<bool, dbus::Error>;
    fn get_locked(&self) -> Result<bool, dbus::Error>;
    fn get_password_mode(&self) -> Result<i32, dbus::Error>;
    fn get_password_hint(&self) -> Result<String, dbus::Error>;
    fn get_automatic_login(&self) -> Result<bool, dbus::Error>;
    fn get_system_account(&self) -> Result<bool, dbus::Error>;
    fn get_local_account(&self) -> Result<bool, dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target=blocking::Connection>> AccountsUser for blocking::Proxy<'a, C> {

    fn set_user_name(&self, name: &str) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.Accounts.User", "SetUserName", (name, ))
    }

    fn set_real_name(&self, name: &str) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.Accounts.User", "SetRealName", (name, ))
    }

    fn set_email(&self, email: &str) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.Accounts.User", "SetEmail", (email, ))
    }

    fn set_language(&self, language: &str) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.Accounts.User", "SetLanguage", (language, ))
    }

    fn set_formats_locale(&self, formats_locale: &str) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.Accounts.User", "SetFormatsLocale", (formats_locale, ))
    }

    fn set_input_sources(&self, sources: Vec<::std::collections::HashMap<&str, &str>>) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.Accounts.User", "SetInputSources", (sources, ))
    }

    fn set_xsession(&self, x_session: &str) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.Accounts.User", "SetXSession", (x_session, ))
    }

    fn set_session(&self, session: &str) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.Accounts.User", "SetSession", (session, ))
    }

    fn set_session_type(&self, session_type: &str) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.Accounts.User", "SetSessionType", (session_type, ))
    }

    fn set_location(&self, location: &str) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.Accounts.User", "SetLocation", (location, ))
    }

    fn set_home_directory(&self, homedir: &str) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.Accounts.User", "SetHomeDirectory", (homedir, ))
    }

    fn set_shell(&self, shell: &str) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.Accounts.User", "SetShell", (shell, ))
    }

    fn set_xhas_messages(&self, has_messages: bool) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.Accounts.User", "SetXHasMessages", (has_messages, ))
    }

    fn set_xkeyboard_layouts(&self, layouts: Vec<&str>) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.Accounts.User", "SetXKeyboardLayouts", (layouts, ))
    }

    fn set_background_file(&self, filename: &str) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.Accounts.User", "SetBackgroundFile", (filename, ))
    }

    fn set_icon_file(&self, filename: &str) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.Accounts.User", "SetIconFile", (filename, ))
    }

    fn set_locked(&self, locked: bool) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.Accounts.User", "SetLocked", (locked, ))
    }

    fn set_account_type(&self, account_type: i32) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.Accounts.User", "SetAccountType", (account_type, ))
    }

    fn set_password_mode(&self, mode: i32) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.Accounts.User", "SetPasswordMode", (mode, ))
    }

    fn set_password(&self, password: &str, hint: &str) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.Accounts.User", "SetPassword", (password, hint, ))
    }

    fn set_password_hint(&self, hint: &str) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.Accounts.User", "SetPasswordHint", (hint, ))
    }

    fn set_automatic_login(&self, enabled: bool) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.Accounts.User", "SetAutomaticLogin", (enabled, ))
    }

    fn get_password_expiration_policy(&self) -> Result<(i64, i64, i64, i64, i64, i64), dbus::Error> {
        self.method_call("org.freedesktop.Accounts.User", "GetPasswordExpirationPolicy", ())
    }

    fn get_uid(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.Accounts.User", "Uid")
    }

    fn get_user_name(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.Accounts.User", "UserName")
    }

    fn get_real_name(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.Accounts.User", "RealName")
    }

    fn get_account_type(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.Accounts.User", "AccountType")
    }

    fn get_home_directory(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.Accounts.User", "HomeDirectory")
    }

    fn get_shell(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.Accounts.User", "Shell")
    }

    fn get_email(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.Accounts.User", "Email")
    }

    fn get_language(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.Accounts.User", "Language")
    }

    fn get_session(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.Accounts.User", "Session")
    }

    fn get_session_type(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.Accounts.User", "SessionType")
    }

    fn get_formats_locale(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.Accounts.User", "FormatsLocale")
    }

    fn get_input_sources(&self) -> Result<Vec<::std::collections::HashMap<String, String>>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.Accounts.User", "InputSources")
    }

    fn get_xsession(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.Accounts.User", "XSession")
    }

    fn get_location(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.Accounts.User", "Location")
    }

    fn get_login_frequency(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.Accounts.User", "LoginFrequency")
    }

    fn get_login_time(&self) -> Result<i64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.Accounts.User", "LoginTime")
    }

    fn get_login_history(&self) -> Result<Vec<(i64, i64, ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.Accounts.User", "LoginHistory")
    }

    fn get_xhas_messages(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.Accounts.User", "XHasMessages")
    }

    fn get_xkeyboard_layouts(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.Accounts.User", "XKeyboardLayouts")
    }

    fn get_background_file(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.Accounts.User", "BackgroundFile")
    }

    fn get_icon_file(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.Accounts.User", "IconFile")
    }

    fn get_saved(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.Accounts.User", "Saved")
    }

    fn get_locked(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.Accounts.User", "Locked")
    }

    fn get_password_mode(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.Accounts.User", "PasswordMode")
    }

    fn get_password_hint(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.Accounts.User", "PasswordHint")
    }

    fn get_automatic_login(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.Accounts.User", "AutomaticLogin")
    }

    fn get_system_account(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.Accounts.User", "SystemAccount")
    }

    fn get_local_account(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.Accounts.User", "LocalAccount")
    }
}

#[derive(Debug)]
pub struct AccountsUserChanged {
}

impl arg::AppendAll for AccountsUserChanged {
    fn append(&self, _: &mut arg::IterAppend) {
    }
}

impl arg::ReadAll for AccountsUserChanged {
    fn read(_: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(AccountsUserChanged {
        })
    }
}

impl dbus::message::SignalArgs for AccountsUserChanged {
    const NAME: &'static str = "Changed";
    const INTERFACE: &'static str = "org.freedesktop.Accounts.User";
}