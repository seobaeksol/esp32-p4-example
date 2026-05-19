#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<IntEnaSpec>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<IntEnaSpec>;
#[doc = "Field `SCAN_DONE` reader - need_des"]
pub type ScanDoneR = crate::BitReader;
#[doc = "Field `SCAN_DONE` writer - need_des"]
pub type ScanDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE` reader - need_des"]
pub type DoneR = crate::BitReader;
#[doc = "Field `DONE` writer - need_des"]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE` reader - need_des"]
pub type ActiveR = crate::BitReader;
#[doc = "Field `ACTIVE` writer - need_des"]
pub type ActiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INACTIVE` reader - need_des"]
pub type InactiveR = crate::BitReader;
#[doc = "Field `INACTIVE` writer - need_des"]
pub type InactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT` reader - need_des"]
pub type TimeoutR = crate::BitReader;
#[doc = "Field `TIMEOUT` writer - need_des"]
pub type TimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APPROACH_LOOP_DONE` reader - need_des"]
pub type ApproachLoopDoneR = crate::BitReader;
#[doc = "Field `APPROACH_LOOP_DONE` writer - need_des"]
pub type ApproachLoopDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BASELINE_UPDATE` reader - need_des"]
pub type BaselineUpdateR = crate::BitReader;
#[doc = "Field `BASELINE_UPDATE` writer - need_des"]
pub type BaselineUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn scan_done(&self) -> ScanDoneR {
        ScanDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn inactive(&self) -> InactiveR {
        InactiveR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn approach_loop_done(&self) -> ApproachLoopDoneR {
        ApproachLoopDoneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn baseline_update(&self) -> BaselineUpdateR {
        BaselineUpdateR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn scan_done(&mut self) -> ScanDoneW<'_, IntEnaSpec> {
        ScanDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<'_, IntEnaSpec> {
        DoneW::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn active(&mut self) -> ActiveW<'_, IntEnaSpec> {
        ActiveW::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn inactive(&mut self) -> InactiveW<'_, IntEnaSpec> {
        InactiveW::new(self, 3)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TimeoutW<'_, IntEnaSpec> {
        TimeoutW::new(self, 4)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn approach_loop_done(&mut self) -> ApproachLoopDoneW<'_, IntEnaSpec> {
        ApproachLoopDoneW::new(self, 5)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn baseline_update(&mut self) -> BaselineUpdateW<'_, IntEnaSpec> {
        BaselineUpdateW::new(self, 6)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEnaSpec;
impl crate::RegisterSpec for IntEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for IntEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for IntEnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for IntEnaSpec {}
