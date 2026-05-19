#[doc = "Register `TOUCH_APPROACH_WORK_MEAS_NUM` reader"]
pub type R = crate::R<TouchApproachWorkMeasNumSpec>;
#[doc = "Register `TOUCH_APPROACH_WORK_MEAS_NUM` writer"]
pub type W = crate::W<TouchApproachWorkMeasNumSpec>;
#[doc = "Field `TOUCH_APPROACH_MEAS_NUM2` reader - need_des"]
pub type TouchApproachMeasNum2R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_APPROACH_MEAS_NUM2` writer - need_des"]
pub type TouchApproachMeasNum2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TOUCH_APPROACH_MEAS_NUM1` reader - need_des"]
pub type TouchApproachMeasNum1R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_APPROACH_MEAS_NUM1` writer - need_des"]
pub type TouchApproachMeasNum1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TOUCH_APPROACH_MEAS_NUM0` reader - need_des"]
pub type TouchApproachMeasNum0R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_APPROACH_MEAS_NUM0` writer - need_des"]
pub type TouchApproachMeasNum0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - need_des"]
    #[inline(always)]
    pub fn touch_approach_meas_num2(&self) -> TouchApproachMeasNum2R {
        TouchApproachMeasNum2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - need_des"]
    #[inline(always)]
    pub fn touch_approach_meas_num1(&self) -> TouchApproachMeasNum1R {
        TouchApproachMeasNum1R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - need_des"]
    #[inline(always)]
    pub fn touch_approach_meas_num0(&self) -> TouchApproachMeasNum0R {
        TouchApproachMeasNum0R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - need_des"]
    #[inline(always)]
    pub fn touch_approach_meas_num2(
        &mut self,
    ) -> TouchApproachMeasNum2W<'_, TouchApproachWorkMeasNumSpec> {
        TouchApproachMeasNum2W::new(self, 0)
    }
    #[doc = "Bits 10:19 - need_des"]
    #[inline(always)]
    pub fn touch_approach_meas_num1(
        &mut self,
    ) -> TouchApproachMeasNum1W<'_, TouchApproachWorkMeasNumSpec> {
        TouchApproachMeasNum1W::new(self, 10)
    }
    #[doc = "Bits 20:29 - need_des"]
    #[inline(always)]
    pub fn touch_approach_meas_num0(
        &mut self,
    ) -> TouchApproachMeasNum0W<'_, TouchApproachWorkMeasNumSpec> {
        TouchApproachMeasNum0W::new(self, 20)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_approach_work_meas_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_approach_work_meas_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TouchApproachWorkMeasNumSpec;
impl crate::RegisterSpec for TouchApproachWorkMeasNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_approach_work_meas_num::R`](R) reader structure"]
impl crate::Readable for TouchApproachWorkMeasNumSpec {}
#[doc = "`write(|w| ..)` method takes [`touch_approach_work_meas_num::W`](W) writer structure"]
impl crate::Writable for TouchApproachWorkMeasNumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOUCH_APPROACH_WORK_MEAS_NUM to value 0x0641_9064"]
impl crate::Resettable for TouchApproachWorkMeasNumSpec {
    const RESET_VALUE: u32 = 0x0641_9064;
}
