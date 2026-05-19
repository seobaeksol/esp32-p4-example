#[doc = "Register `DBIAS_CHANNEL_SEL1` reader"]
pub type R = crate::R<DbiasChannelSel1Spec>;
#[doc = "Register `DBIAS_CHANNEL_SEL1` writer"]
pub type W = crate::W<DbiasChannelSel1Spec>;
#[doc = "Field `DBIAS_CHANNEL4_SEL` reader - needs field desc"]
pub type DbiasChannel4SelR = crate::FieldReader;
#[doc = "Field `DBIAS_CHANNEL4_SEL` writer - needs field desc"]
pub type DbiasChannel4SelW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 25:31 - needs field desc"]
    #[inline(always)]
    pub fn dbias_channel4_sel(&self) -> DbiasChannel4SelR {
        DbiasChannel4SelR::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 25:31 - needs field desc"]
    #[inline(always)]
    pub fn dbias_channel4_sel(&mut self) -> DbiasChannel4SelW<'_, DbiasChannelSel1Spec> {
        DbiasChannel4SelW::new(self, 25)
    }
}
#[doc = "needs desc\n\nYou can [`read`](crate::Reg::read) this register and get [`dbias_channel_sel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbias_channel_sel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbiasChannelSel1Spec;
impl crate::RegisterSpec for DbiasChannelSel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbias_channel_sel1::R`](R) reader structure"]
impl crate::Readable for DbiasChannelSel1Spec {}
#[doc = "`write(|w| ..)` method takes [`dbias_channel_sel1::W`](W) writer structure"]
impl crate::Writable for DbiasChannelSel1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBIAS_CHANNEL_SEL1 to value 0x8000_0000"]
impl crate::Resettable for DbiasChannelSel1Spec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
