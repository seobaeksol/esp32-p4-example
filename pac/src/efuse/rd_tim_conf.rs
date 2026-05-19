#[doc = "Register `RD_TIM_CONF` reader"]
pub type R = crate::R<RdTimConfSpec>;
#[doc = "Register `RD_TIM_CONF` writer"]
pub type W = crate::W<RdTimConfSpec>;
#[doc = "Field `THR_A` reader - Configures the read hold time."]
pub type ThrAR = crate::FieldReader;
#[doc = "Field `THR_A` writer - Configures the read hold time."]
pub type ThrAW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRD` reader - Configures the read time."]
pub type TrdR = crate::FieldReader;
#[doc = "Field `TRD` writer - Configures the read time."]
pub type TrdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TSUR_A` reader - Configures the read setup time."]
pub type TsurAR = crate::FieldReader;
#[doc = "Field `TSUR_A` writer - Configures the read setup time."]
pub type TsurAW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `READ_INIT_NUM` reader - Configures the waiting time of reading eFuse memory."]
pub type ReadInitNumR = crate::FieldReader;
#[doc = "Field `READ_INIT_NUM` writer - Configures the waiting time of reading eFuse memory."]
pub type ReadInitNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Configures the read hold time."]
    #[inline(always)]
    pub fn thr_a(&self) -> ThrAR {
        ThrAR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Configures the read time."]
    #[inline(always)]
    pub fn trd(&self) -> TrdR {
        TrdR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Configures the read setup time."]
    #[inline(always)]
    pub fn tsur_a(&self) -> TsurAR {
        TsurAR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Configures the waiting time of reading eFuse memory."]
    #[inline(always)]
    pub fn read_init_num(&self) -> ReadInitNumR {
        ReadInitNumR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures the read hold time."]
    #[inline(always)]
    pub fn thr_a(&mut self) -> ThrAW<'_, RdTimConfSpec> {
        ThrAW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Configures the read time."]
    #[inline(always)]
    pub fn trd(&mut self) -> TrdW<'_, RdTimConfSpec> {
        TrdW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Configures the read setup time."]
    #[inline(always)]
    pub fn tsur_a(&mut self) -> TsurAW<'_, RdTimConfSpec> {
        TsurAW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Configures the waiting time of reading eFuse memory."]
    #[inline(always)]
    pub fn read_init_num(&mut self) -> ReadInitNumW<'_, RdTimConfSpec> {
        ReadInitNumW::new(self, 24)
    }
}
#[doc = "Configures read timing parameters.\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_tim_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_tim_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdTimConfSpec;
impl crate::RegisterSpec for RdTimConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_tim_conf::R`](R) reader structure"]
impl crate::Readable for RdTimConfSpec {}
#[doc = "`write(|w| ..)` method takes [`rd_tim_conf::W`](W) writer structure"]
impl crate::Writable for RdTimConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RD_TIM_CONF to value 0x0f01_0201"]
impl crate::Resettable for RdTimConfSpec {
    const RESET_VALUE: u32 = 0x0f01_0201;
}
