#[doc = "Register `CORE_AHB_TIMEOUT` reader"]
pub type R = crate::R<CoreAhbTimeoutSpec>;
#[doc = "Register `CORE_AHB_TIMEOUT` writer"]
pub type W = crate::W<CoreAhbTimeoutSpec>;
#[doc = "Field `EN` reader - set this field to 1 to enable hp core0&1 ahb timeout handle"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - set this field to 1 to enable hp core0&1 ahb timeout handle"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THRES` reader - This field used to set hp core0&1 ahb bus timeout threshold"]
pub type ThresR = crate::FieldReader<u16>;
#[doc = "Field `THRES` writer - This field used to set hp core0&1 ahb bus timeout threshold"]
pub type ThresW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - set this field to 1 to enable hp core0&1 ahb timeout handle"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:16 - This field used to set hp core0&1 ahb bus timeout threshold"]
    #[inline(always)]
    pub fn thres(&self) -> ThresR {
        ThresR::new(((self.bits >> 1) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - set this field to 1 to enable hp core0&1 ahb timeout handle"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CoreAhbTimeoutSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 1:16 - This field used to set hp core0&1 ahb bus timeout threshold"]
    #[inline(always)]
    pub fn thres(&mut self) -> ThresW<'_, CoreAhbTimeoutSpec> {
        ThresW::new(self, 1)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`core_ahb_timeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_ahb_timeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CoreAhbTimeoutSpec;
impl crate::RegisterSpec for CoreAhbTimeoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_ahb_timeout::R`](R) reader structure"]
impl crate::Readable for CoreAhbTimeoutSpec {}
#[doc = "`write(|w| ..)` method takes [`core_ahb_timeout::W`](W) writer structure"]
impl crate::Writable for CoreAhbTimeoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_AHB_TIMEOUT to value 0x0001_ffff"]
impl crate::Resettable for CoreAhbTimeoutSpec {
    const RESET_VALUE: u32 = 0x0001_ffff;
}
