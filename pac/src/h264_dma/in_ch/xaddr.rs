#[doc = "Register `XADDR` reader"]
pub type R = crate::R<XaddrSpec>;
#[doc = "Field `IN_CMDFIFO_XADDR` reader - only for debug"]
pub type InCmdfifoXaddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - only for debug"]
    #[inline(always)]
    pub fn in_cmdfifo_xaddr(&self) -> InCmdfifoXaddrR {
        InCmdfifoXaddrR::new(self.bits)
    }
}
#[doc = "RX CHx xaddr register\n\nYou can [`read`](crate::Reg::read) this register and get [`xaddr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XaddrSpec;
impl crate::RegisterSpec for XaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xaddr::R`](R) reader structure"]
impl crate::Readable for XaddrSpec {}
#[doc = "`reset()` method sets XADDR to value 0"]
impl crate::Resettable for XaddrSpec {}
