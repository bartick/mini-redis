

#[cfg(test)]
mod test {
    use std::sync::Mutex;

    use lazy_static::lazy_static;
    use crate::create_connection::get_connection;
    use redis::{Commands, Connection, ConnectionLike};

    lazy_static! {
        static ref CONN: Mutex<Connection> = Mutex::new(get_connection());
    }

    #[test]
    fn test_get_connection() {
        let conn = CONN.lock().unwrap();
        assert_eq!(conn.is_open(), true);
    }

    #[test]
    fn test_set_and_get() {
        let mut conn = CONN.lock().unwrap();
        let _ : () = conn.set("bartick", 42).unwrap_or_else(|e| {
            panic!("Failed to set key: {}", e);
        });
        let value: Option<i128> = conn.get("bartick").unwrap_or_else(|e| {
            panic!("Failed to get key: {}", e);
        });
        assert_eq!(value, Some(42));
    }

    #[test]
    fn test_get_nonexistent() {
        let mut conn = CONN.lock().unwrap();
        let value: Option<i128> = conn.get("nonexistent").unwrap_or_else(|e| {
            panic!("Failed to get key: {}", e);
        });
        assert_eq!(value, None);
    }

    #[test]
    fn test_delete() {
        let mut conn = CONN.lock().unwrap();
        let _ : () = conn.set("bartick", 42).unwrap_or_else(|e| {
            panic!("Failed to set key: {}", e);
        });
        let _ : () = conn.del("bartick").unwrap_or_else(|e| {
            panic!("Failed to delete key: {}", e);
        });
        let value: Option<i128> = conn.get("bartick").unwrap_or_else(|e| {
            panic!("Failed to get key: {}", e);
        });
        assert_eq!(value, None);
    }

    #[test]
    fn test_incr() {
        let mut conn = CONN.lock().unwrap();
        let _ : () = conn.set("bartick", 42).unwrap_or_else(|e| {
            panic!("Failed to set key: {}", e);
        });
        let _ : () = conn.incr("bartick", 1).unwrap_or_else(|e| {
            panic!("Failed to increment key: {}", e);
        });
        let value: Option<i128> = conn.get("bartick").unwrap_or_else(|e| {
            panic!("Failed to get key: {}", e);
        });
        assert_eq!(value, Some(43));
    }

    #[test]
    fn test_decr() {
        let mut conn = CONN.lock().unwrap();
        let _ : () = conn.set("bartick", 42).unwrap_or_else(|e| {
            panic!("Failed to set key: {}", e);
        });
        let _ : () = conn.decr("bartick", 1).unwrap_or_else(|e| {
            panic!("Failed to decrement key: {}", e);
        });
        let value: Option<i128> = conn.get("bartick").unwrap_or_else(|e| {
            panic!("Failed to get key: {}", e);
        });
        assert_eq!(value, Some(41));
    }

}