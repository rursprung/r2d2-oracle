#![deny(missing_docs)]

//! Oracle support for the r2d2 connection pool.

pub use oracle;
pub use r2d2;

/// The configuration used for `oracle::Connection::connect`.
#[derive(Debug)]
struct Config {
    username: String,
    password: String,
    connect_string: String,
    params: Vec<oracle::ConnParam>,
}

/// An `r2d2::ManageConnection` for `oracle::Connection`s.
///
/// # Example
/// ```no_run
/// use std::thread;
/// use r2d2_oracle::OracleConnectionManager;
///
/// let manager = OracleConnectionManager::new("user", "password", "localhost", &[]);
/// let pool = r2d2::Pool::builder()
///      .max_size(15)
///      .build(manager)
///      .unwrap();
///
/// for _ in 0..20 {
///   let pool = pool.clone();
///   thread::spawn(move || {
///     let conn = pool.get().unwrap();
///     // use the connection
///     // it will be returned to the pool when it falls out of scope.
///   });
/// }
/// ```
#[derive(Debug)]
pub struct OracleConnectionManager {
    config: Config,
}

impl OracleConnectionManager {
    /// Initialise the connection manager with the data needed to create new connections.
    ///
    /// # Example
    /// ```
    /// # use r2d2_oracle::OracleConnectionManager;
    /// let manager = OracleConnectionManager::new("user", "password", "localhost", &[]);
    /// ```
    pub fn new(username: &str, password: &str, connect_string: &str, params: &[oracle::ConnParam]) -> OracleConnectionManager {
        OracleConnectionManager {
            config: Config {
                username: String::from(username),
                password: String::from(password),
                connect_string: String::from(connect_string),
                params: Vec::from(params)
            }
        }
    }
}

impl r2d2::ManageConnection for OracleConnectionManager {
    type Connection = oracle::Connection;
    type Error = oracle::Error;

    fn connect(&self) -> Result<oracle::Connection, oracle::Error> {
        oracle::Connection::connect(&self.config.username, &self.config.password, &self.config.connect_string, &self.config.params.as_slice())
    }

    fn is_valid(&self, conn: &mut oracle::Connection) -> Result<(), oracle::Error> {
        conn.query("SELECT 1 FROM dual", &[]).map(|_| ())
    }

    fn has_broken(&self, conn: &mut oracle::Connection) -> bool {
        self.is_valid(conn).is_err()
    }
}

