#[doc = "Register `BUF_HB_RCV` reader"]
pub type R = crate::R<BufHbRcvSpec>;
#[doc = "Field `IN_CMDFIFO_BUF_HB_RCV` reader - only for debug"]
pub type InCmdfifoBufHbRcvR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:28 - only for debug"]
    #[inline(always)]
    pub fn in_cmdfifo_buf_hb_rcv(&self) -> InCmdfifoBufHbRcvR {
        InCmdfifoBufHbRcvR::new(self.bits & 0x1fff_ffff)
    }
}
#[doc = "rx CH5 buf len hb rcv register\n\nYou can [`read`](crate::Reg::read) this register and get [`buf_hb_rcv::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BufHbRcvSpec;
impl crate::RegisterSpec for BufHbRcvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf_hb_rcv::R`](R) reader structure"]
impl crate::Readable for BufHbRcvSpec {}
#[doc = "`reset()` method sets BUF_HB_RCV to value 0"]
impl crate::Resettable for BufHbRcvSpec {}
