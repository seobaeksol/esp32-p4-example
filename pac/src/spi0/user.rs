#[doc = "Register `USER` reader"]
pub type R = crate::R<UserSpec>;
#[doc = "Register `USER` writer"]
pub type W = crate::W<UserSpec>;
#[doc = "Field `CS_HOLD` reader - spi cs keep low when spi is in done phase. 1: enable 0: disable."]
pub type CsHoldR = crate::BitReader;
#[doc = "Field `CS_HOLD` writer - spi cs keep low when spi is in done phase. 1: enable 0: disable."]
pub type CsHoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_SETUP` reader - spi cs is enable when spi is in prepare phase. 1: enable 0: disable."]
pub type CsSetupR = crate::BitReader;
#[doc = "Field `CS_SETUP` writer - spi cs is enable when spi is in prepare phase. 1: enable 0: disable."]
pub type CsSetupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_OUT_EDGE` reader - The bit combined with SPI_MEM_CK_IDLE_EDGE bit to control SPI clock mode 0~3."]
pub type CkOutEdgeR = crate::BitReader;
#[doc = "Field `CK_OUT_EDGE` writer - The bit combined with SPI_MEM_CK_IDLE_EDGE bit to control SPI clock mode 0~3."]
pub type CkOutEdgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_DUMMY_IDLE` reader - spi clock is disable in dummy phase when the bit is enable."]
pub type UsrDummyIdleR = crate::BitReader;
#[doc = "Field `USR_DUMMY_IDLE` writer - spi clock is disable in dummy phase when the bit is enable."]
pub type UsrDummyIdleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_DUMMY` reader - This bit enable the dummy phase of an operation."]
pub type UsrDummyR = crate::BitReader;
#[doc = "Field `USR_DUMMY` writer - This bit enable the dummy phase of an operation."]
pub type UsrDummyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - spi cs keep low when spi is in done phase. 1: enable 0: disable."]
    #[inline(always)]
    pub fn cs_hold(&self) -> CsHoldR {
        CsHoldR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - spi cs is enable when spi is in prepare phase. 1: enable 0: disable."]
    #[inline(always)]
    pub fn cs_setup(&self) -> CsSetupR {
        CsSetupR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - The bit combined with SPI_MEM_CK_IDLE_EDGE bit to control SPI clock mode 0~3."]
    #[inline(always)]
    pub fn ck_out_edge(&self) -> CkOutEdgeR {
        CkOutEdgeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 26 - spi clock is disable in dummy phase when the bit is enable."]
    #[inline(always)]
    pub fn usr_dummy_idle(&self) -> UsrDummyIdleR {
        UsrDummyIdleR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - This bit enable the dummy phase of an operation."]
    #[inline(always)]
    pub fn usr_dummy(&self) -> UsrDummyR {
        UsrDummyR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - spi cs keep low when spi is in done phase. 1: enable 0: disable."]
    #[inline(always)]
    pub fn cs_hold(&mut self) -> CsHoldW<'_, UserSpec> {
        CsHoldW::new(self, 6)
    }
    #[doc = "Bit 7 - spi cs is enable when spi is in prepare phase. 1: enable 0: disable."]
    #[inline(always)]
    pub fn cs_setup(&mut self) -> CsSetupW<'_, UserSpec> {
        CsSetupW::new(self, 7)
    }
    #[doc = "Bit 9 - The bit combined with SPI_MEM_CK_IDLE_EDGE bit to control SPI clock mode 0~3."]
    #[inline(always)]
    pub fn ck_out_edge(&mut self) -> CkOutEdgeW<'_, UserSpec> {
        CkOutEdgeW::new(self, 9)
    }
    #[doc = "Bit 26 - spi clock is disable in dummy phase when the bit is enable."]
    #[inline(always)]
    pub fn usr_dummy_idle(&mut self) -> UsrDummyIdleW<'_, UserSpec> {
        UsrDummyIdleW::new(self, 26)
    }
    #[doc = "Bit 29 - This bit enable the dummy phase of an operation."]
    #[inline(always)]
    pub fn usr_dummy(&mut self) -> UsrDummyW<'_, UserSpec> {
        UsrDummyW::new(self, 29)
    }
}
#[doc = "SPI0 user register.\n\nYou can [`read`](crate::Reg::read) this register and get [`user::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UserSpec;
impl crate::RegisterSpec for UserSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`user::R`](R) reader structure"]
impl crate::Readable for UserSpec {}
#[doc = "`write(|w| ..)` method takes [`user::W`](W) writer structure"]
impl crate::Writable for UserSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USER to value 0"]
impl crate::Resettable for UserSpec {}
