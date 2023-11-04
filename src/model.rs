use core::cmp::{Ord, Ordering, PartialOrd};
use hash32::{Hash, Hasher};
use heapless::FnvIndexSet;

#[derive(Copy, Clone, Debug, Eq)]
pub enum Connection {
    Top(usize, usize),
    Bottom(usize, usize),
}

impl PartialOrd for Connection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Connection {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Connection::Top(a, b) => match other {
                Connection::Top(p, q) => {
                    if p.min(q) > a.min(b) && p.max(q) > a.max(b) {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                }
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
}

impl Connection {
    pub fn reflexive(&self) -> bool {
        match self {
            Connection::Top(a, b) => a == b,
            Connection::Bottom(a, b) => a == b,
        }
    }

    pub fn non_overlapping(&self, other: &Self) -> bool {
        match self {
            Connection::Top(a, b) => match other {
                Connection::Top(p, q) => p.min(q) > a.max(b) || a.min(b) > p.max(q),
                _ => true,
            },
            Connection::Bottom(a, b) => match other {
                Connection::Bottom(p, q) => p.min(q) > a.max(b) || a.min(b) > p.max(q),
                _ => true,
            },
        }
    }
}

impl Hash for Connection {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Connection::Top(a, b) => {
                state.write(&[1]);
                state.write(&(*a.min(b)).to_ne_bytes());
                state.write(&(*a.max(b)).to_ne_bytes());
            }
            Connection::Bottom(a, b) => {
                state.write(&[2]);
                state.write(&(*a.min(b)).to_ne_bytes());
                state.write(&(*a.max(b)).to_ne_bytes());
            }
        }
    }
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Connection::Top(a, b) => match other {
                Connection::Top(p, q) => (a == p && b == q) || (b == p && a == q),
                _ => false,
            },
            Connection::Bottom(a, b) => match other {
                Connection::Bottom(p, q) => (a == p && b == q) || (b == p && a == q),
                _ => false,
            },
        }
    }
}

#[derive(Clone)]
pub struct Connections(FnvIndexSet<Connection, 32>);

impl Connections {
    pub fn new() -> Self {
        Self(FnvIndexSet::new())
    }

    pub fn remove(&mut self, connection: Connection) {
        assert!(!connection.reflexive());
        let _ = self.0.remove(&connection);
    }

    pub fn insert(&mut self, connection: Connection) {
        assert!(!connection.reflexive());
        let _ = self.0.insert(connection);
    }

    pub fn iter(&mut self) -> impl Iterator<Item = &Connection> {
        self.0.iter()
    }
}
