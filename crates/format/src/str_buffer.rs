#[derive(Clone, PartialEq, Eq)]
pub struct StrBuffer<const N: usize> {
    buf: [u8; N],
    len: usize,
}

impl<const N: usize> StrBuffer<N> {
    pub fn new() -> Self {
        StrBuffer {
            buf: [0; N],
            len: 0,
        }
    }

    pub fn push_str<'s>(&mut self, s: &'s str) -> Result<(), &'s str> {
        let rem_len = self.buf.len() - self.len;
        if rem_len < s.len() {
            Err(s)
        } else {
            let start = self.len;
            let end = start + s.len();
            self.buf[start..end].copy_from_slice(s.as_bytes());
            self.len += s.len();
            Ok(())
        }
    }

    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.buf[..self.len]) }
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn first(&self) -> Option<&str> {
        if self.is_empty() {
            None
        } else {
            self.as_str().split("").filter(|c| !c.is_empty()).next()
        }
    }

    pub fn strip_first(&mut self) -> Option<()> {
        let first_len = self.first()?.len();
        self.buf.copy_within(first_len..self.len, 0);
        self.len -= first_len;
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXHAUSTED: &str = "Buffer has been exhausted";

    #[test]
    fn test_push_str() {
        let mut buf = StrBuffer::<5>::new();
        buf.push_str("&").expect(EXHAUSTED);
        buf.push_str("amp").expect(EXHAUSTED);
        buf.push_str(";").expect(EXHAUSTED);
        assert_eq!("&amp;", buf.as_str());
    }

    #[test]
    fn test_exhaustion() {
        let mut buf = StrBuffer::<5>::new();
        buf.push_str("&amp;").expect(EXHAUSTED);
        assert_eq!(Err(" "), buf.push_str(" "));
    }

    #[test]
    fn test_clear() {
        let mut buf = StrBuffer::<5>::new();
        buf.push_str("&amp;").expect(EXHAUSTED);
        assert_eq!("&amp;", buf.as_str());
        buf.clear();
        assert_eq!("", buf.as_str());
    }

    #[test]
    fn test_length() {
        let mut buf = StrBuffer::<5>::new();
        assert_eq!(buf.len(), 0);
        buf.push_str("&").expect(EXHAUSTED);
        assert_eq!(buf.len(), 1);
        buf.push_str("amp").expect(EXHAUSTED);
        assert_eq!(buf.len(), 4);
        buf.push_str(";").expect(EXHAUSTED);
        assert_eq!(buf.len(), 5);
        buf.clear();
        assert_eq!(buf.len(), 0);
    }

    #[test]
    fn test_is_empty() {
        let mut buf = StrBuffer::<5>::new();
        assert!(buf.is_empty());
        buf.push_str("&").expect(EXHAUSTED);
        assert!(!buf.is_empty());
        buf.push_str("amp").expect(EXHAUSTED);
        assert!(!buf.is_empty());
        buf.push_str(";").expect(EXHAUSTED);
        assert!(!buf.is_empty());
        buf.clear();
        assert!(buf.is_empty());
    }

    #[test]
    fn test_first() {
        let mut buf = StrBuffer::<5>::new();
        assert_eq!(None, buf.first());
        buf.push_str("&").expect(EXHAUSTED);
        assert_eq!(Some("&"), buf.first());
        buf.push_str("amp").expect(EXHAUSTED);
        assert_eq!(Some("&"), buf.first());
        buf.push_str(";").expect(EXHAUSTED);
        assert_eq!(Some("&"), buf.first());
        buf.clear();
        assert_eq!(None, buf.first());
    }

    #[test]
    fn test_strip_first() {
        let mut buf = StrBuffer::<8>::new();
        assert_eq!(None, buf.strip_first());
        buf.push_str("&&amp;").expect(EXHAUSTED);
        assert_eq!(Some("&"), buf.first());
        assert_eq!(Some(()), buf.strip_first());
        assert_eq!("&amp;", buf.as_str());
    }

    #[test]
    fn test_strip_first_wide() {
        let mut buf = StrBuffer::<8>::new();
        assert_eq!(None, buf.strip_first());
        buf.push_str("АБВ").expect(EXHAUSTED);
        assert_eq!(Some("А"), buf.first());
        assert_eq!(Some(()), buf.strip_first());
        assert_eq!("БВ", buf.as_str());
    }
}
