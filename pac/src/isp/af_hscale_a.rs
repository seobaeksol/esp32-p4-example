#[doc = "Register `AF_HSCALE_A` reader"]
pub type R = crate::R<AfHscaleASpec>;
#[doc = "Register `AF_HSCALE_A` writer"]
pub type W = crate::W<AfHscaleASpec>;
#[doc = "Field `AF_RPOINT_A` reader - this field configures left coordinate of focus window a, must >= 2"]
pub type AfRpointAR = crate::FieldReader<u16>;
#[doc = "Field `AF_RPOINT_A` writer - this field configures left coordinate of focus window a, must >= 2"]
pub type AfRpointAW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `AF_LPOINT_A` reader - this field configures top coordinate of focus window a, must >= 2"]
pub type AfLpointAR = crate::FieldReader<u16>;
#[doc = "Field `AF_LPOINT_A` writer - this field configures top coordinate of focus window a, must >= 2"]
pub type AfLpointAW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - this field configures left coordinate of focus window a, must >= 2"]
    #[inline(always)]
    pub fn af_rpoint_a(&self) -> AfRpointAR {
        AfRpointAR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - this field configures top coordinate of focus window a, must >= 2"]
    #[inline(always)]
    pub fn af_lpoint_a(&self) -> AfLpointAR {
        AfLpointAR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - this field configures left coordinate of focus window a, must >= 2"]
    #[inline(always)]
    pub fn af_rpoint_a(&mut self) -> AfRpointAW<'_, AfHscaleASpec> {
        AfRpointAW::new(self, 0)
    }
    #[doc = "Bits 16:27 - this field configures top coordinate of focus window a, must >= 2"]
    #[inline(always)]
    pub fn af_lpoint_a(&mut self) -> AfLpointAW<'_, AfHscaleASpec> {
        AfLpointAW::new(self, 16)
    }
}
#[doc = "h-scale of af window a register\n\nYou can [`read`](crate::Reg::read) this register and get [`af_hscale_a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af_hscale_a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfHscaleASpec;
impl crate::RegisterSpec for AfHscaleASpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af_hscale_a::R`](R) reader structure"]
impl crate::Readable for AfHscaleASpec {}
#[doc = "`write(|w| ..)` method takes [`af_hscale_a::W`](W) writer structure"]
impl crate::Writable for AfHscaleASpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AF_HSCALE_A to value 0x0001_0080"]
impl crate::Resettable for AfHscaleASpec {
    const RESET_VALUE: u32 = 0x0001_0080;
}
