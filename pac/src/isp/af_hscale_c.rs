#[doc = "Register `AF_HSCALE_C` reader"]
pub type R = crate::R<AfHscaleCSpec>;
#[doc = "Register `AF_HSCALE_C` writer"]
pub type W = crate::W<AfHscaleCSpec>;
#[doc = "Field `AF_RPOINT_C` reader - this field configures left coordinate of focus window c, must >= 2"]
pub type AfRpointCR = crate::FieldReader<u16>;
#[doc = "Field `AF_RPOINT_C` writer - this field configures left coordinate of focus window c, must >= 2"]
pub type AfRpointCW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `AF_LPOINT_C` reader - this field configures top coordinate of focus window c, must >= 2"]
pub type AfLpointCR = crate::FieldReader<u16>;
#[doc = "Field `AF_LPOINT_C` writer - this field configures top coordinate of focus window c, must >= 2"]
pub type AfLpointCW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - this field configures left coordinate of focus window c, must >= 2"]
    #[inline(always)]
    pub fn af_rpoint_c(&self) -> AfRpointCR {
        AfRpointCR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - this field configures top coordinate of focus window c, must >= 2"]
    #[inline(always)]
    pub fn af_lpoint_c(&self) -> AfLpointCR {
        AfLpointCR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - this field configures left coordinate of focus window c, must >= 2"]
    #[inline(always)]
    pub fn af_rpoint_c(&mut self) -> AfRpointCW<'_, AfHscaleCSpec> {
        AfRpointCW::new(self, 0)
    }
    #[doc = "Bits 16:27 - this field configures top coordinate of focus window c, must >= 2"]
    #[inline(always)]
    pub fn af_lpoint_c(&mut self) -> AfLpointCW<'_, AfHscaleCSpec> {
        AfLpointCW::new(self, 16)
    }
}
#[doc = "v-scale of af window c register\n\nYou can [`read`](crate::Reg::read) this register and get [`af_hscale_c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af_hscale_c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfHscaleCSpec;
impl crate::RegisterSpec for AfHscaleCSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af_hscale_c::R`](R) reader structure"]
impl crate::Readable for AfHscaleCSpec {}
#[doc = "`write(|w| ..)` method takes [`af_hscale_c::W`](W) writer structure"]
impl crate::Writable for AfHscaleCSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AF_HSCALE_C to value 0x0001_0080"]
impl crate::Resettable for AfHscaleCSpec {
    const RESET_VALUE: u32 = 0x0001_0080;
}
