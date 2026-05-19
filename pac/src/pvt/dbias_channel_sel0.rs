#[doc = "Register `DBIAS_CHANNEL_SEL0` reader"]
pub type R = crate::R<DbiasChannelSel0Spec>;
#[doc = "Register `DBIAS_CHANNEL_SEL0` writer"]
pub type W = crate::W<DbiasChannelSel0Spec>;
#[doc = "Field `DBIAS_CHANNEL3_SEL` reader - needs field desc"]
pub type DbiasChannel3SelR = crate::FieldReader;
#[doc = "Field `DBIAS_CHANNEL3_SEL` writer - needs field desc"]
pub type DbiasChannel3SelW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DBIAS_CHANNEL2_SEL` reader - needs field desc"]
pub type DbiasChannel2SelR = crate::FieldReader;
#[doc = "Field `DBIAS_CHANNEL2_SEL` writer - needs field desc"]
pub type DbiasChannel2SelW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DBIAS_CHANNEL1_SEL` reader - needs field desc"]
pub type DbiasChannel1SelR = crate::FieldReader;
#[doc = "Field `DBIAS_CHANNEL1_SEL` writer - needs field desc"]
pub type DbiasChannel1SelW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DBIAS_CHANNEL0_SEL` reader - needs field desc"]
pub type DbiasChannel0SelR = crate::FieldReader;
#[doc = "Field `DBIAS_CHANNEL0_SEL` writer - needs field desc"]
pub type DbiasChannel0SelW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 4:10 - needs field desc"]
    #[inline(always)]
    pub fn dbias_channel3_sel(&self) -> DbiasChannel3SelR {
        DbiasChannel3SelR::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:17 - needs field desc"]
    #[inline(always)]
    pub fn dbias_channel2_sel(&self) -> DbiasChannel2SelR {
        DbiasChannel2SelR::new(((self.bits >> 11) & 0x7f) as u8)
    }
    #[doc = "Bits 18:24 - needs field desc"]
    #[inline(always)]
    pub fn dbias_channel1_sel(&self) -> DbiasChannel1SelR {
        DbiasChannel1SelR::new(((self.bits >> 18) & 0x7f) as u8)
    }
    #[doc = "Bits 25:31 - needs field desc"]
    #[inline(always)]
    pub fn dbias_channel0_sel(&self) -> DbiasChannel0SelR {
        DbiasChannel0SelR::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:10 - needs field desc"]
    #[inline(always)]
    pub fn dbias_channel3_sel(&mut self) -> DbiasChannel3SelW<'_, DbiasChannelSel0Spec> {
        DbiasChannel3SelW::new(self, 4)
    }
    #[doc = "Bits 11:17 - needs field desc"]
    #[inline(always)]
    pub fn dbias_channel2_sel(&mut self) -> DbiasChannel2SelW<'_, DbiasChannelSel0Spec> {
        DbiasChannel2SelW::new(self, 11)
    }
    #[doc = "Bits 18:24 - needs field desc"]
    #[inline(always)]
    pub fn dbias_channel1_sel(&mut self) -> DbiasChannel1SelW<'_, DbiasChannelSel0Spec> {
        DbiasChannel1SelW::new(self, 18)
    }
    #[doc = "Bits 25:31 - needs field desc"]
    #[inline(always)]
    pub fn dbias_channel0_sel(&mut self) -> DbiasChannel0SelW<'_, DbiasChannelSel0Spec> {
        DbiasChannel0SelW::new(self, 25)
    }
}
#[doc = "needs desc\n\nYou can [`read`](crate::Reg::read) this register and get [`dbias_channel_sel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbias_channel_sel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbiasChannelSel0Spec;
impl crate::RegisterSpec for DbiasChannelSel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbias_channel_sel0::R`](R) reader structure"]
impl crate::Readable for DbiasChannelSel0Spec {}
#[doc = "`write(|w| ..)` method takes [`dbias_channel_sel0::W`](W) writer structure"]
impl crate::Writable for DbiasChannelSel0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBIAS_CHANNEL_SEL0 to value 0x8102_0400"]
impl crate::Resettable for DbiasChannelSel0Spec {
    const RESET_VALUE: u32 = 0x8102_0400;
}
