

# [doc = "FDCAN Message RAM"]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Fdcanram { ptr : * mut u8 } unsafe impl Send for Fdcanram { } unsafe impl Sync for Fdcanram { } impl Fdcanram { # [inline (always)]
pub const unsafe fn from_ptr (ptr : * mut ()) -> Self { Self { ptr : ptr as _ , } } # [inline (always)]
pub const fn as_ptr (& self) -> * mut () { self . ptr as _ } # [doc = "11-bit filter"]
# [inline (always)]
pub const fn flssa (self , n : usize) -> crate :: common :: Reg < u32 , crate :: common :: RW > { assert ! (n < 28usize) ; unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0usize + n * 4usize) as _) } } # [doc = "29-bit filter"]
# [inline (always)]
pub const fn flesa (self , n : usize) -> crate :: common :: Reg < u32 , crate :: common :: RW > { assert ! (n < 16usize) ; unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (112usize + n * 4usize) as _) } } # [doc = "Rx FIFO 0"]
# [inline (always)]
pub const fn rxfifo0 (self , n : usize) -> crate :: common :: Reg < u32 , crate :: common :: RW > { assert ! (n < 54usize) ; unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (176usize + n * 4usize) as _) } } # [doc = "Rx FIFO 1"]
# [inline (always)]
pub const fn rxfifo1 (self , n : usize) -> crate :: common :: Reg < u32 , crate :: common :: RW > { assert ! (n < 54usize) ; unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (392usize + n * 4usize) as _) } } # [doc = "Tx event FIFO"]
# [inline (always)]
pub const fn txefifo (self , n : usize) -> crate :: common :: Reg < u32 , crate :: common :: RW > { assert ! (n < 6usize) ; unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (608usize + n * 4usize) as _) } } # [doc = "Tx buffer"]
# [inline (always)]
pub const fn txbuf (self , n : usize) -> crate :: common :: Reg < u32 , crate :: common :: RW > { assert ! (n < 54usize) ; unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (632usize + n * 4usize) as _) } } }