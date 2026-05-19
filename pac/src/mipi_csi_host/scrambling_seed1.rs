#[doc = "Register `SCRAMBLING_SEED1` reader"]
pub type R = crate::R<ScramblingSeed1Spec>;
#[doc = "Register `SCRAMBLING_SEED1` writer"]
pub type W = crate::W<ScramblingSeed1Spec>;
#[doc = "Field `SCRAMBLE_SEED_LANE1` reader - NA"]
pub type ScrambleSeedLane1R = crate::FieldReader<u16>;
#[doc = "Field `SCRAMBLE_SEED_LANE1` writer - NA"]
pub type ScrambleSeedLane1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - NA"]
    #[inline(always)]
    pub fn scramble_seed_lane1(&self) -> ScrambleSeedLane1R {
        ScrambleSeedLane1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - NA"]
    #[inline(always)]
    pub fn scramble_seed_lane1(&mut self) -> ScrambleSeedLane1W<'_, ScramblingSeed1Spec> {
        ScrambleSeedLane1W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`scrambling_seed1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scrambling_seed1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScramblingSeed1Spec;
impl crate::RegisterSpec for ScramblingSeed1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scrambling_seed1::R`](R) reader structure"]
impl crate::Readable for ScramblingSeed1Spec {}
#[doc = "`write(|w| ..)` method takes [`scrambling_seed1::W`](W) writer structure"]
impl crate::Writable for ScramblingSeed1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCRAMBLING_SEED1 to value 0x1008"]
impl crate::Resettable for ScramblingSeed1Spec {
    const RESET_VALUE: u32 = 0x1008;
}
