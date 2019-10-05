#![deny(missing_docs)]

//! Oracle support for the r2d2 connection pool.

pub use oracle;
pub use r2d2;

/// An `r2d2::ManageConnection` for `oracle::Connection`s.
///
/// # Example
/// ```no_run
/// use std::thread;
/// use r2d2_oracle::OracleConnectionManager;
///
/// let manager = OracleConnectionManager::new("user", "password", "localhost");
/// let pool = r2d2::Pool::builder()
///      .max_size(15)
///      .build(manager)
///      .unwrap();
///
/// for _ in 0..20 {
///     let pool = pool.clone();
///     thread::spawn(move || {
///         let conn = pool.get().unwrap();
///         // use the connection
///         // it will be returned to the pool when it falls out of scope.
///     });
/// }
/// ```
#[derive(Debug)]
pub struct OracleConnectionManager {
    connector: oracle::Connector,
}

impl OracleConnectionManager {
    /// Initialise the connection manager with the data needed to create new connections.
    /// Refer to the documentation of `oracle::Connection` for further details on the parameters.
    ///
    /// # Example
    /// ```
    /// # use r2d2_oracle::OracleConnectionManager;
    /// let manager = OracleConnectionManager::new("user", "password", "localhost");
    /// ```
    pub fn new(username: &str, password: &str, connect_string: &str) -> OracleConnectionManager {
        OracleConnectionManager {
            connector: oracle::Connector::new(username, password, connect_string),
        }
    }

    /// Initialise the connection manager with the data needed to create new connections using `oracle::Connector`.
    /// This allows setting additional connection data.
    ///
    /// If a connection can be established only with a username, password and connect string, use `new` instead.
    ///
    /// # Example
    /// ```
    /// # use r2d2_oracle::OracleConnectionManager;
    /// // connect system/manager as sysdba
    /// let mut connector = oracle::Connector::new("system", "manager", "");
    /// connector.privilege(oracle::Privilege::Sysdba);
    /// let manager = OracleConnectionManager::from_connector(connector);
    /// ```
    pub fn from_connector(connector: oracle::Connector) -> OracleConnectionManager {
        OracleConnectionManager { connector }
    }
}

impl r2d2::ManageConnection for OracleConnectionManager {
    type Connection = oracle::Connection;
    type Error = oracle::Error;

    fn connect(&self) -> Result<oracle::Connection, oracle::Error> {
        self.connector.connect()
    }

    fn is_valid(&self, conn: &mut oracle::Connection) -> Result<(), oracle::Error> {
        conn.ping()
    }

    fn has_broken(&self, conn: &mut oracle::Connection) -> bool {
        match conn.status() {
            Ok(oracle::ConnStatus::Normal) => false,
            _ => true,
        }
    }
}
