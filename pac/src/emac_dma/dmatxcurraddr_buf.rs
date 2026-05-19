#[doc = "Register `DMATXCURRADDR_BUF` reader"]
pub type R = crate::R<DmatxcurraddrBufSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatxcurraddr_buf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmatxcurraddrBufSpec;
impl crate::RegisterSpec for DmatxcurraddrBufSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatxcurraddr_buf::R`](R) reader structure"]
impl crate::Readable for DmatxcurraddrBufSpec {}
#[doc = "`reset()` method sets DMATXCURRADDR_BUF to value 0"]
impl crate::Resettable for DmatxcurraddrBufSpec {}
