#[doc = "Register `L1_CACHE_ACS_CNT_CTRL` reader"]
pub type R = crate::R<L1CacheAcsCntCtrlSpec>;
#[doc = "Register `L1_CACHE_ACS_CNT_CTRL` writer"]
pub type W = crate::W<L1CacheAcsCntCtrlSpec>;
#[doc = "Field `L1_IBUS0_CNT_ENA` reader - The bit is used to enable ibus0 counter in L1-ICache0."]
pub type L1Ibus0CntEnaR = crate::BitReader;
#[doc = "Field `L1_IBUS0_CNT_ENA` writer - The bit is used to enable ibus0 counter in L1-ICache0."]
pub type L1Ibus0CntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_IBUS1_CNT_ENA` reader - The bit is used to enable ibus1 counter in L1-ICache1."]
pub type L1Ibus1CntEnaR = crate::BitReader;
#[doc = "Field `L1_IBUS1_CNT_ENA` writer - The bit is used to enable ibus1 counter in L1-ICache1."]
pub type L1Ibus1CntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_IBUS2_CNT_ENA` reader - Reserved"]
pub type L1Ibus2CntEnaR = crate::BitReader;
#[doc = "Field `L1_IBUS3_CNT_ENA` reader - Reserved"]
pub type L1Ibus3CntEnaR = crate::BitReader;
#[doc = "Field `L1_DBUS0_CNT_ENA` reader - The bit is used to enable dbus0 counter in L1-DCache."]
pub type L1Dbus0CntEnaR = crate::BitReader;
#[doc = "Field `L1_DBUS0_CNT_ENA` writer - The bit is used to enable dbus0 counter in L1-DCache."]
pub type L1Dbus0CntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DBUS1_CNT_ENA` reader - The bit is used to enable dbus1 counter in L1-DCache."]
pub type L1Dbus1CntEnaR = crate::BitReader;
#[doc = "Field `L1_DBUS1_CNT_ENA` writer - The bit is used to enable dbus1 counter in L1-DCache."]
pub type L1Dbus1CntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DBUS2_CNT_ENA` reader - Reserved"]
pub type L1Dbus2CntEnaR = crate::BitReader;
#[doc = "Field `L1_DBUS3_CNT_ENA` reader - Reserved"]
pub type L1Dbus3CntEnaR = crate::BitReader;
#[doc = "Field `L1_IBUS0_CNT_CLR` writer - The bit is used to clear ibus0 counter in L1-ICache0."]
pub type L1Ibus0CntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_IBUS1_CNT_CLR` writer - The bit is used to clear ibus1 counter in L1-ICache1."]
pub type L1Ibus1CntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_IBUS2_CNT_CLR` reader - Reserved"]
pub type L1Ibus2CntClrR = crate::BitReader;
#[doc = "Field `L1_IBUS3_CNT_CLR` reader - Reserved"]
pub type L1Ibus3CntClrR = crate::BitReader;
#[doc = "Field `L1_DBUS0_CNT_CLR` writer - The bit is used to clear dbus0 counter in L1-DCache."]
pub type L1Dbus0CntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DBUS1_CNT_CLR` writer - The bit is used to clear dbus1 counter in L1-DCache."]
pub type L1Dbus1CntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DBUS2_CNT_CLR` reader - Reserved"]
pub type L1Dbus2CntClrR = crate::BitReader;
#[doc = "Field `L1_DBUS3_CNT_CLR` reader - Reserved"]
pub type L1Dbus3CntClrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The bit is used to enable ibus0 counter in L1-ICache0."]
    #[inline(always)]
    pub fn l1_ibus0_cnt_ena(&self) -> L1Ibus0CntEnaR {
        L1Ibus0CntEnaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable ibus1 counter in L1-ICache1."]
    #[inline(always)]
    pub fn l1_ibus1_cnt_ena(&self) -> L1Ibus1CntEnaR {
        L1Ibus1CntEnaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn l1_ibus2_cnt_ena(&self) -> L1Ibus2CntEnaR {
        L1Ibus2CntEnaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1_ibus3_cnt_ena(&self) -> L1Ibus3CntEnaR {
        L1Ibus3CntEnaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to enable dbus0 counter in L1-DCache."]
    #[inline(always)]
    pub fn l1_dbus0_cnt_ena(&self) -> L1Dbus0CntEnaR {
        L1Dbus0CntEnaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The bit is used to enable dbus1 counter in L1-DCache."]
    #[inline(always)]
    pub fn l1_dbus1_cnt_ena(&self) -> L1Dbus1CntEnaR {
        L1Dbus1CntEnaR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn l1_dbus2_cnt_ena(&self) -> L1Dbus2CntEnaR {
        L1Dbus2CntEnaR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn l1_dbus3_cnt_ena(&self) -> L1Dbus3CntEnaR {
        L1Dbus3CntEnaR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    pub fn l1_ibus2_cnt_clr(&self) -> L1Ibus2CntClrR {
        L1Ibus2CntClrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline(always)]
    pub fn l1_ibus3_cnt_clr(&self) -> L1Ibus3CntClrR {
        L1Ibus3CntClrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - Reserved"]
    #[inline(always)]
    pub fn l1_dbus2_cnt_clr(&self) -> L1Dbus2CntClrR {
        L1Dbus2CntClrR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reserved"]
    #[inline(always)]
    pub fn l1_dbus3_cnt_clr(&self) -> L1Dbus3CntClrR {
        L1Dbus3CntClrR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable ibus0 counter in L1-ICache0."]
    #[inline(always)]
    pub fn l1_ibus0_cnt_ena(&mut self) -> L1Ibus0CntEnaW<'_, L1CacheAcsCntCtrlSpec> {
        L1Ibus0CntEnaW::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to enable ibus1 counter in L1-ICache1."]
    #[inline(always)]
    pub fn l1_ibus1_cnt_ena(&mut self) -> L1Ibus1CntEnaW<'_, L1CacheAcsCntCtrlSpec> {
        L1Ibus1CntEnaW::new(self, 1)
    }
    #[doc = "Bit 4 - The bit is used to enable dbus0 counter in L1-DCache."]
    #[inline(always)]
    pub fn l1_dbus0_cnt_ena(&mut self) -> L1Dbus0CntEnaW<'_, L1CacheAcsCntCtrlSpec> {
        L1Dbus0CntEnaW::new(self, 4)
    }
    #[doc = "Bit 5 - The bit is used to enable dbus1 counter in L1-DCache."]
    #[inline(always)]
    pub fn l1_dbus1_cnt_ena(&mut self) -> L1Dbus1CntEnaW<'_, L1CacheAcsCntCtrlSpec> {
        L1Dbus1CntEnaW::new(self, 5)
    }
    #[doc = "Bit 16 - The bit is used to clear ibus0 counter in L1-ICache0."]
    #[inline(always)]
    pub fn l1_ibus0_cnt_clr(&mut self) -> L1Ibus0CntClrW<'_, L1CacheAcsCntCtrlSpec> {
        L1Ibus0CntClrW::new(self, 16)
    }
    #[doc = "Bit 17 - The bit is used to clear ibus1 counter in L1-ICache1."]
    #[inline(always)]
    pub fn l1_ibus1_cnt_clr(&mut self) -> L1Ibus1CntClrW<'_, L1CacheAcsCntCtrlSpec> {
        L1Ibus1CntClrW::new(self, 17)
    }
    #[doc = "Bit 20 - The bit is used to clear dbus0 counter in L1-DCache."]
    #[inline(always)]
    pub fn l1_dbus0_cnt_clr(&mut self) -> L1Dbus0CntClrW<'_, L1CacheAcsCntCtrlSpec> {
        L1Dbus0CntClrW::new(self, 20)
    }
    #[doc = "Bit 21 - The bit is used to clear dbus1 counter in L1-DCache."]
    #[inline(always)]
    pub fn l1_dbus1_cnt_clr(&mut self) -> L1Dbus1CntClrW<'_, L1CacheAcsCntCtrlSpec> {
        L1Dbus1CntClrW::new(self, 21)
    }
}
#[doc = "Cache Access Counter enable and clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_cache_acs_cnt_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_cache_acs_cnt_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1CacheAcsCntCtrlSpec;
impl crate::RegisterSpec for L1CacheAcsCntCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_cache_acs_cnt_ctrl::R`](R) reader structure"]
impl crate::Readable for L1CacheAcsCntCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`l1_cache_acs_cnt_ctrl::W`](W) writer structure"]
impl crate::Writable for L1CacheAcsCntCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1_CACHE_ACS_CNT_CTRL to value 0"]
impl crate::Resettable for L1CacheAcsCntCtrlSpec {}
