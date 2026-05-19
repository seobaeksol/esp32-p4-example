#[doc = "Register `DMAIN_EN` reader"]
pub type R = crate::R<DmainEnSpec>;
#[doc = "Register `DMAIN_EN` writer"]
pub type W = crate::W<DmainEnSpec>;
#[doc = "Field `DMAIN_TIE` reader - When this bit is set with Normal Interrupt Summary Enable (Bit\\[16\\]) the Transmit Interrupt is enabled. When this bit is reset the Transmit Interrupt is disabled."]
pub type DmainTieR = crate::BitReader;
#[doc = "Field `DMAIN_TIE` writer - When this bit is set with Normal Interrupt Summary Enable (Bit\\[16\\]) the Transmit Interrupt is enabled. When this bit is reset the Transmit Interrupt is disabled."]
pub type DmainTieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAIN_TSE` reader - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Transmission Stopped Interrupt is enabled. When this bit is reset the Transmission Stopped Interrupt is disabled."]
pub type DmainTseR = crate::BitReader;
#[doc = "Field `DMAIN_TSE` writer - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Transmission Stopped Interrupt is enabled. When this bit is reset the Transmission Stopped Interrupt is disabled."]
pub type DmainTseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAIN_TBUE` reader - When this bit is set with Normal Interrupt Summary Enable (Bit 16) the Transmit Buffer Unavailable Interrupt is enabled. When this bit is reset the Transmit Buffer Unavailable Interrupt is Disabled."]
pub type DmainTbueR = crate::BitReader;
#[doc = "Field `DMAIN_TBUE` writer - When this bit is set with Normal Interrupt Summary Enable (Bit 16) the Transmit Buffer Unavailable Interrupt is enabled. When this bit is reset the Transmit Buffer Unavailable Interrupt is Disabled."]
pub type DmainTbueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAIN_TJTE` reader - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Transmit Jabber Timeout Interrupt is enabled. When this bit is reset the Transmit Jabber Timeout Interrupt is disabled."]
pub type DmainTjteR = crate::BitReader;
#[doc = "Field `DMAIN_TJTE` writer - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Transmit Jabber Timeout Interrupt is enabled. When this bit is reset the Transmit Jabber Timeout Interrupt is disabled."]
pub type DmainTjteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAIN_OIE` reader - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Receive Overflow Interrupt is enabled. When this bit is reset the Overflow Interrupt is disabled."]
pub type DmainOieR = crate::BitReader;
#[doc = "Field `DMAIN_OIE` writer - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Receive Overflow Interrupt is enabled. When this bit is reset the Overflow Interrupt is disabled."]
pub type DmainOieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAIN_UIE` reader - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Transmit Underflow Interrupt is enabled. When this bit is reset the Underflow Interrupt is disabled."]
pub type DmainUieR = crate::BitReader;
#[doc = "Field `DMAIN_UIE` writer - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Transmit Underflow Interrupt is enabled. When this bit is reset the Underflow Interrupt is disabled."]
pub type DmainUieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAIN_RIE` reader - When this bit is set with Normal Interrupt Summary Enable (Bit\\[16\\]) the Receive Interrupt is enabled. When this bit is reset the Receive Interrupt is disabled."]
pub type DmainRieR = crate::BitReader;
#[doc = "Field `DMAIN_RIE` writer - When this bit is set with Normal Interrupt Summary Enable (Bit\\[16\\]) the Receive Interrupt is enabled. When this bit is reset the Receive Interrupt is disabled."]
pub type DmainRieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAIN_RBUE` reader - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Receive Buffer Unavailable Interrupt is enabled. When this bit is reset the Receive Buffer Unavailable Interrupt is disabled."]
pub type DmainRbueR = crate::BitReader;
#[doc = "Field `DMAIN_RBUE` writer - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Receive Buffer Unavailable Interrupt is enabled. When this bit is reset the Receive Buffer Unavailable Interrupt is disabled."]
pub type DmainRbueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAIN_RSE` reader - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Receive Stopped Interrupt is enabled. When this bit is reset the Receive Stopped Interrupt is disabled."]
pub type DmainRseR = crate::BitReader;
#[doc = "Field `DMAIN_RSE` writer - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Receive Stopped Interrupt is enabled. When this bit is reset the Receive Stopped Interrupt is disabled."]
pub type DmainRseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAIN_RWTE` reader - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Receive Watchdog Timeout Interrupt is enabled. When this bit is reset the Receive Watchdog Timeout Interrupt is disabled."]
pub type DmainRwteR = crate::BitReader;
#[doc = "Field `DMAIN_RWTE` writer - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Receive Watchdog Timeout Interrupt is enabled. When this bit is reset the Receive Watchdog Timeout Interrupt is disabled."]
pub type DmainRwteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAIN_ETIE` reader - When this bit is set with an Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Early Transmit Interrupt is enabled. When this bit is reset the Early Transmit Interrupt is disabled."]
pub type DmainEtieR = crate::BitReader;
#[doc = "Field `DMAIN_ETIE` writer - When this bit is set with an Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Early Transmit Interrupt is enabled. When this bit is reset the Early Transmit Interrupt is disabled."]
pub type DmainEtieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAIN_FBEE` reader - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Fatal Bus Error Interrupt is enabled. When this bit is reset the Fatal Bus Error Enable Interrupt is disabled."]
pub type DmainFbeeR = crate::BitReader;
#[doc = "Field `DMAIN_FBEE` writer - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Fatal Bus Error Interrupt is enabled. When this bit is reset the Fatal Bus Error Enable Interrupt is disabled."]
pub type DmainFbeeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAIN_ERIE` reader - When this bit is set with Normal Interrupt Summary Enable (Bit\\[16\\]) the Early Receive Interrupt is enabled. When this bit is reset the Early Receive Interrupt is disabled."]
pub type DmainErieR = crate::BitReader;
#[doc = "Field `DMAIN_ERIE` writer - When this bit is set with Normal Interrupt Summary Enable (Bit\\[16\\]) the Early Receive Interrupt is enabled. When this bit is reset the Early Receive Interrupt is disabled."]
pub type DmainErieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAIN_AISE` reader - When this bit is set abnormal interrupt summary is enabled. When this bit is reset the abnormal interrupt summary is disabled. This bit enables the following interrupts in Status Register: Bit\\[1\\]: Transmit Process Stopped. Bit\\[3\\]: Transmit Jabber Timeout. Bit\\[4\\]: Receive Overflow. Bit\\[5\\]: Transmit Underflow. Bit\\[7\\]: Receive Buffer Unavailable. Bit\\[8\\]: Receive Process Stopped. Bit\\[9\\]: Receive Watchdog Timeout. Bit\\[10\\]: Early Transmit Interrupt. Bit\\[13\\]: Fatal Bus Error."]
pub type DmainAiseR = crate::BitReader;
#[doc = "Field `DMAIN_AISE` writer - When this bit is set abnormal interrupt summary is enabled. When this bit is reset the abnormal interrupt summary is disabled. This bit enables the following interrupts in Status Register: Bit\\[1\\]: Transmit Process Stopped. Bit\\[3\\]: Transmit Jabber Timeout. Bit\\[4\\]: Receive Overflow. Bit\\[5\\]: Transmit Underflow. Bit\\[7\\]: Receive Buffer Unavailable. Bit\\[8\\]: Receive Process Stopped. Bit\\[9\\]: Receive Watchdog Timeout. Bit\\[10\\]: Early Transmit Interrupt. Bit\\[13\\]: Fatal Bus Error."]
pub type DmainAiseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAIN_NISE` reader - When this bit is set normal interrupt summary is enabled. When this bit is reset normal interrupt summary is disabled. This bit enables the following interrupts in Status Register: Bit\\[0\\]: Transmit Interrupt. Bit\\[2\\]: Transmit Buffer Unavailable. Bit\\[6\\]: Receive Interrupt. Bit\\[14\\]: Early Receive Interrupt."]
pub type DmainNiseR = crate::BitReader;
#[doc = "Field `DMAIN_NISE` writer - When this bit is set normal interrupt summary is enabled. When this bit is reset normal interrupt summary is disabled. This bit enables the following interrupts in Status Register: Bit\\[0\\]: Transmit Interrupt. Bit\\[2\\]: Transmit Buffer Unavailable. Bit\\[6\\]: Receive Interrupt. Bit\\[14\\]: Early Receive Interrupt."]
pub type DmainNiseW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When this bit is set with Normal Interrupt Summary Enable (Bit\\[16\\]) the Transmit Interrupt is enabled. When this bit is reset the Transmit Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_tie(&self) -> DmainTieR {
        DmainTieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Transmission Stopped Interrupt is enabled. When this bit is reset the Transmission Stopped Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_tse(&self) -> DmainTseR {
        DmainTseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When this bit is set with Normal Interrupt Summary Enable (Bit 16) the Transmit Buffer Unavailable Interrupt is enabled. When this bit is reset the Transmit Buffer Unavailable Interrupt is Disabled."]
    #[inline(always)]
    pub fn dmain_tbue(&self) -> DmainTbueR {
        DmainTbueR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Transmit Jabber Timeout Interrupt is enabled. When this bit is reset the Transmit Jabber Timeout Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_tjte(&self) -> DmainTjteR {
        DmainTjteR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Receive Overflow Interrupt is enabled. When this bit is reset the Overflow Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_oie(&self) -> DmainOieR {
        DmainOieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Transmit Underflow Interrupt is enabled. When this bit is reset the Underflow Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_uie(&self) -> DmainUieR {
        DmainUieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When this bit is set with Normal Interrupt Summary Enable (Bit\\[16\\]) the Receive Interrupt is enabled. When this bit is reset the Receive Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_rie(&self) -> DmainRieR {
        DmainRieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Receive Buffer Unavailable Interrupt is enabled. When this bit is reset the Receive Buffer Unavailable Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_rbue(&self) -> DmainRbueR {
        DmainRbueR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Receive Stopped Interrupt is enabled. When this bit is reset the Receive Stopped Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_rse(&self) -> DmainRseR {
        DmainRseR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Receive Watchdog Timeout Interrupt is enabled. When this bit is reset the Receive Watchdog Timeout Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_rwte(&self) -> DmainRwteR {
        DmainRwteR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When this bit is set with an Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Early Transmit Interrupt is enabled. When this bit is reset the Early Transmit Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_etie(&self) -> DmainEtieR {
        DmainEtieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Fatal Bus Error Interrupt is enabled. When this bit is reset the Fatal Bus Error Enable Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_fbee(&self) -> DmainFbeeR {
        DmainFbeeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - When this bit is set with Normal Interrupt Summary Enable (Bit\\[16\\]) the Early Receive Interrupt is enabled. When this bit is reset the Early Receive Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_erie(&self) -> DmainErieR {
        DmainErieR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - When this bit is set abnormal interrupt summary is enabled. When this bit is reset the abnormal interrupt summary is disabled. This bit enables the following interrupts in Status Register: Bit\\[1\\]: Transmit Process Stopped. Bit\\[3\\]: Transmit Jabber Timeout. Bit\\[4\\]: Receive Overflow. Bit\\[5\\]: Transmit Underflow. Bit\\[7\\]: Receive Buffer Unavailable. Bit\\[8\\]: Receive Process Stopped. Bit\\[9\\]: Receive Watchdog Timeout. Bit\\[10\\]: Early Transmit Interrupt. Bit\\[13\\]: Fatal Bus Error."]
    #[inline(always)]
    pub fn dmain_aise(&self) -> DmainAiseR {
        DmainAiseR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - When this bit is set normal interrupt summary is enabled. When this bit is reset normal interrupt summary is disabled. This bit enables the following interrupts in Status Register: Bit\\[0\\]: Transmit Interrupt. Bit\\[2\\]: Transmit Buffer Unavailable. Bit\\[6\\]: Receive Interrupt. Bit\\[14\\]: Early Receive Interrupt."]
    #[inline(always)]
    pub fn dmain_nise(&self) -> DmainNiseR {
        DmainNiseR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When this bit is set with Normal Interrupt Summary Enable (Bit\\[16\\]) the Transmit Interrupt is enabled. When this bit is reset the Transmit Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_tie(&mut self) -> DmainTieW<'_, DmainEnSpec> {
        DmainTieW::new(self, 0)
    }
    #[doc = "Bit 1 - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Transmission Stopped Interrupt is enabled. When this bit is reset the Transmission Stopped Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_tse(&mut self) -> DmainTseW<'_, DmainEnSpec> {
        DmainTseW::new(self, 1)
    }
    #[doc = "Bit 2 - When this bit is set with Normal Interrupt Summary Enable (Bit 16) the Transmit Buffer Unavailable Interrupt is enabled. When this bit is reset the Transmit Buffer Unavailable Interrupt is Disabled."]
    #[inline(always)]
    pub fn dmain_tbue(&mut self) -> DmainTbueW<'_, DmainEnSpec> {
        DmainTbueW::new(self, 2)
    }
    #[doc = "Bit 3 - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Transmit Jabber Timeout Interrupt is enabled. When this bit is reset the Transmit Jabber Timeout Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_tjte(&mut self) -> DmainTjteW<'_, DmainEnSpec> {
        DmainTjteW::new(self, 3)
    }
    #[doc = "Bit 4 - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Receive Overflow Interrupt is enabled. When this bit is reset the Overflow Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_oie(&mut self) -> DmainOieW<'_, DmainEnSpec> {
        DmainOieW::new(self, 4)
    }
    #[doc = "Bit 5 - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Transmit Underflow Interrupt is enabled. When this bit is reset the Underflow Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_uie(&mut self) -> DmainUieW<'_, DmainEnSpec> {
        DmainUieW::new(self, 5)
    }
    #[doc = "Bit 6 - When this bit is set with Normal Interrupt Summary Enable (Bit\\[16\\]) the Receive Interrupt is enabled. When this bit is reset the Receive Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_rie(&mut self) -> DmainRieW<'_, DmainEnSpec> {
        DmainRieW::new(self, 6)
    }
    #[doc = "Bit 7 - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Receive Buffer Unavailable Interrupt is enabled. When this bit is reset the Receive Buffer Unavailable Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_rbue(&mut self) -> DmainRbueW<'_, DmainEnSpec> {
        DmainRbueW::new(self, 7)
    }
    #[doc = "Bit 8 - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Receive Stopped Interrupt is enabled. When this bit is reset the Receive Stopped Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_rse(&mut self) -> DmainRseW<'_, DmainEnSpec> {
        DmainRseW::new(self, 8)
    }
    #[doc = "Bit 9 - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Receive Watchdog Timeout Interrupt is enabled. When this bit is reset the Receive Watchdog Timeout Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_rwte(&mut self) -> DmainRwteW<'_, DmainEnSpec> {
        DmainRwteW::new(self, 9)
    }
    #[doc = "Bit 10 - When this bit is set with an Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Early Transmit Interrupt is enabled. When this bit is reset the Early Transmit Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_etie(&mut self) -> DmainEtieW<'_, DmainEnSpec> {
        DmainEtieW::new(self, 10)
    }
    #[doc = "Bit 13 - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Fatal Bus Error Interrupt is enabled. When this bit is reset the Fatal Bus Error Enable Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_fbee(&mut self) -> DmainFbeeW<'_, DmainEnSpec> {
        DmainFbeeW::new(self, 13)
    }
    #[doc = "Bit 14 - When this bit is set with Normal Interrupt Summary Enable (Bit\\[16\\]) the Early Receive Interrupt is enabled. When this bit is reset the Early Receive Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_erie(&mut self) -> DmainErieW<'_, DmainEnSpec> {
        DmainErieW::new(self, 14)
    }
    #[doc = "Bit 15 - When this bit is set abnormal interrupt summary is enabled. When this bit is reset the abnormal interrupt summary is disabled. This bit enables the following interrupts in Status Register: Bit\\[1\\]: Transmit Process Stopped. Bit\\[3\\]: Transmit Jabber Timeout. Bit\\[4\\]: Receive Overflow. Bit\\[5\\]: Transmit Underflow. Bit\\[7\\]: Receive Buffer Unavailable. Bit\\[8\\]: Receive Process Stopped. Bit\\[9\\]: Receive Watchdog Timeout. Bit\\[10\\]: Early Transmit Interrupt. Bit\\[13\\]: Fatal Bus Error."]
    #[inline(always)]
    pub fn dmain_aise(&mut self) -> DmainAiseW<'_, DmainEnSpec> {
        DmainAiseW::new(self, 15)
    }
    #[doc = "Bit 16 - When this bit is set normal interrupt summary is enabled. When this bit is reset normal interrupt summary is disabled. This bit enables the following interrupts in Status Register: Bit\\[0\\]: Transmit Interrupt. Bit\\[2\\]: Transmit Buffer Unavailable. Bit\\[6\\]: Receive Interrupt. Bit\\[14\\]: Early Receive Interrupt."]
    #[inline(always)]
    pub fn dmain_nise(&mut self) -> DmainNiseW<'_, DmainEnSpec> {
        DmainNiseW::new(self, 16)
    }
}
#[doc = "Enable / disable interrupts\n\nYou can [`read`](crate::Reg::read) this register and get [`dmain_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmain_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmainEnSpec;
impl crate::RegisterSpec for DmainEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmain_en::R`](R) reader structure"]
impl crate::Readable for DmainEnSpec {}
#[doc = "`write(|w| ..)` method takes [`dmain_en::W`](W) writer structure"]
impl crate::Writable for DmainEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMAIN_EN to value 0"]
impl crate::Resettable for DmainEnSpec {}
