#[doc = "Register `SCL_EXT_LOW_TIME` reader"]
pub type R = crate::R<SclExtLowTimeSpec>;
#[doc = "Register `SCL_EXT_LOW_TIME` writer"]
pub type W = crate::W<SclExtLowTimeSpec>;
#[doc = "Field `REG_I3C_MST_EXT_LOW_PERIOD1` reader - NA"]
pub type RegI3cMstExtLowPeriod1R = crate::FieldReader;
#[doc = "Field `REG_I3C_MST_EXT_LOW_PERIOD1` writer - NA"]
pub type RegI3cMstExtLowPeriod1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_I3C_MST_EXT_LOW_PERIOD2` reader - NA"]
pub type RegI3cMstExtLowPeriod2R = crate::FieldReader;
#[doc = "Field `REG_I3C_MST_EXT_LOW_PERIOD2` writer - NA"]
pub type RegI3cMstExtLowPeriod2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_I3C_MST_EXT_LOW_PERIOD3` reader - NA"]
pub type RegI3cMstExtLowPeriod3R = crate::FieldReader;
#[doc = "Field `REG_I3C_MST_EXT_LOW_PERIOD3` writer - NA"]
pub type RegI3cMstExtLowPeriod3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_I3C_MST_EXT_LOW_PERIOD4` reader - NA"]
pub type RegI3cMstExtLowPeriod4R = crate::FieldReader;
#[doc = "Field `REG_I3C_MST_EXT_LOW_PERIOD4` writer - NA"]
pub type RegI3cMstExtLowPeriod4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    pub fn reg_i3c_mst_ext_low_period1(&self) -> RegI3cMstExtLowPeriod1R {
        RegI3cMstExtLowPeriod1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - NA"]
    #[inline(always)]
    pub fn reg_i3c_mst_ext_low_period2(&self) -> RegI3cMstExtLowPeriod2R {
        RegI3cMstExtLowPeriod2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - NA"]
    #[inline(always)]
    pub fn reg_i3c_mst_ext_low_period3(&self) -> RegI3cMstExtLowPeriod3R {
        RegI3cMstExtLowPeriod3R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - NA"]
    #[inline(always)]
    pub fn reg_i3c_mst_ext_low_period4(&self) -> RegI3cMstExtLowPeriod4R {
        RegI3cMstExtLowPeriod4R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    pub fn reg_i3c_mst_ext_low_period1(
        &mut self,
    ) -> RegI3cMstExtLowPeriod1W<'_, SclExtLowTimeSpec> {
        RegI3cMstExtLowPeriod1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - NA"]
    #[inline(always)]
    pub fn reg_i3c_mst_ext_low_period2(
        &mut self,
    ) -> RegI3cMstExtLowPeriod2W<'_, SclExtLowTimeSpec> {
        RegI3cMstExtLowPeriod2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - NA"]
    #[inline(always)]
    pub fn reg_i3c_mst_ext_low_period3(
        &mut self,
    ) -> RegI3cMstExtLowPeriod3W<'_, SclExtLowTimeSpec> {
        RegI3cMstExtLowPeriod3W::new(self, 16)
    }
    #[doc = "Bits 24:31 - NA"]
    #[inline(always)]
    pub fn reg_i3c_mst_ext_low_period4(
        &mut self,
    ) -> RegI3cMstExtLowPeriod4W<'_, SclExtLowTimeSpec> {
        RegI3cMstExtLowPeriod4W::new(self, 24)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`scl_ext_low_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl_ext_low_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SclExtLowTimeSpec;
impl crate::RegisterSpec for SclExtLowTimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scl_ext_low_time::R`](R) reader structure"]
impl crate::Readable for SclExtLowTimeSpec {}
#[doc = "`write(|w| ..)` method takes [`scl_ext_low_time::W`](W) writer structure"]
impl crate::Writable for SclExtLowTimeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCL_EXT_LOW_TIME to value 0"]
impl crate::Resettable for SclExtLowTimeSpec {}
