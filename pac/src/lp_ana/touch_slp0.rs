#[doc = "Register `TOUCH_SLP0` reader"]
pub type R = crate::R<TouchSlp0Spec>;
#[doc = "Register `TOUCH_SLP0` writer"]
pub type W = crate::W<TouchSlp0Spec>;
#[doc = "Field `TOUCH_SLP_TH0` reader - need_des"]
pub type TouchSlpTh0R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_SLP_TH0` writer - need_des"]
pub type TouchSlpTh0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TOUCH_SLP_CHANNEL_CLR` writer - need_des"]
pub type TouchSlpChannelClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_SLP_PAD` reader - need_des"]
pub type TouchSlpPadR = crate::FieldReader;
#[doc = "Field `TOUCH_SLP_PAD` writer - need_des"]
pub type TouchSlpPadW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn touch_slp_th0(&self) -> TouchSlpTh0R {
        TouchSlpTh0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 17:20 - need_des"]
    #[inline(always)]
    pub fn touch_slp_pad(&self) -> TouchSlpPadR {
        TouchSlpPadR::new(((self.bits >> 17) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn touch_slp_th0(&mut self) -> TouchSlpTh0W<'_, TouchSlp0Spec> {
        TouchSlpTh0W::new(self, 0)
    }
    #[doc = "Bit 16 - need_des"]
    #[inline(always)]
    pub fn touch_slp_channel_clr(&mut self) -> TouchSlpChannelClrW<'_, TouchSlp0Spec> {
        TouchSlpChannelClrW::new(self, 16)
    }
    #[doc = "Bits 17:20 - need_des"]
    #[inline(always)]
    pub fn touch_slp_pad(&mut self) -> TouchSlpPadW<'_, TouchSlp0Spec> {
        TouchSlpPadW::new(self, 17)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_slp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_slp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TouchSlp0Spec;
impl crate::RegisterSpec for TouchSlp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_slp0::R`](R) reader structure"]
impl crate::Readable for TouchSlp0Spec {}
#[doc = "`write(|w| ..)` method takes [`touch_slp0::W`](W) writer structure"]
impl crate::Writable for TouchSlp0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOUCH_SLP0 to value 0x001e_0000"]
impl crate::Resettable for TouchSlp0Spec {
    const RESET_VALUE: u32 = 0x001e_0000;
}
