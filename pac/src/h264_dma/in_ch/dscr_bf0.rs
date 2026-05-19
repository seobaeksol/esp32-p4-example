#[doc = "Register `DSCR_BF0` reader"]
pub type R = crate::R<DscrBf0Spec>;
#[doc = "Field `INLINK_DSCR_BF0` reader - The address of the last inlink descriptor's next address x-1."]
pub type InlinkDscrBf0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The address of the last inlink descriptor's next address x-1."]
    #[inline(always)]
    pub fn inlink_dscr_bf0(&self) -> InlinkDscrBf0R {
        InlinkDscrBf0R::new(self.bits)
    }
}
#[doc = "RX CHx last dscr addr register\n\nYou can [`read`](crate::Reg::read) this register and get [`dscr_bf0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DscrBf0Spec;
impl crate::RegisterSpec for DscrBf0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dscr_bf0::R`](R) reader structure"]
impl crate::Readable for DscrBf0Spec {}
#[doc = "`reset()` method sets DSCR_BF0 to value 0"]
impl crate::Resettable for DscrBf0Spec {}
