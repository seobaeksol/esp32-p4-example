#[doc = "Register `TOUCH_WORK` reader"]
pub type R = crate::R<TouchWorkSpec>;
#[doc = "Register `TOUCH_WORK` writer"]
pub type W = crate::W<TouchWorkSpec>;
#[doc = "Field `DIV_NUM2` reader - need_des"]
pub type DivNum2R = crate::FieldReader;
#[doc = "Field `DIV_NUM2` writer - need_des"]
pub type DivNum2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DIV_NUM1` reader - need_des"]
pub type DivNum1R = crate::FieldReader;
#[doc = "Field `DIV_NUM1` writer - need_des"]
pub type DivNum1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DIV_NUM0` reader - need_des"]
pub type DivNum0R = crate::FieldReader;
#[doc = "Field `DIV_NUM0` writer - need_des"]
pub type DivNum0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TOUCH_OUT_SEL` reader - need_des"]
pub type TouchOutSelR = crate::BitReader;
#[doc = "Field `TOUCH_OUT_SEL` writer - need_des"]
pub type TouchOutSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_OUT_RESET` writer - need_des"]
pub type TouchOutResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_OUT_GATE` reader - need_des"]
pub type TouchOutGateR = crate::BitReader;
#[doc = "Field `TOUCH_OUT_GATE` writer - need_des"]
pub type TouchOutGateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:18 - need_des"]
    #[inline(always)]
    pub fn div_num2(&self) -> DivNum2R {
        DivNum2R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - need_des"]
    #[inline(always)]
    pub fn div_num1(&self) -> DivNum1R {
        DivNum1R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24 - need_des"]
    #[inline(always)]
    pub fn div_num0(&self) -> DivNum0R {
        DivNum0R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn touch_out_sel(&self) -> TouchOutSelR {
        TouchOutSelR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn touch_out_gate(&self) -> TouchOutGateR {
        TouchOutGateR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:18 - need_des"]
    #[inline(always)]
    pub fn div_num2(&mut self) -> DivNum2W<'_, TouchWorkSpec> {
        DivNum2W::new(self, 16)
    }
    #[doc = "Bits 19:21 - need_des"]
    #[inline(always)]
    pub fn div_num1(&mut self) -> DivNum1W<'_, TouchWorkSpec> {
        DivNum1W::new(self, 19)
    }
    #[doc = "Bits 22:24 - need_des"]
    #[inline(always)]
    pub fn div_num0(&mut self) -> DivNum0W<'_, TouchWorkSpec> {
        DivNum0W::new(self, 22)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn touch_out_sel(&mut self) -> TouchOutSelW<'_, TouchWorkSpec> {
        TouchOutSelW::new(self, 25)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn touch_out_reset(&mut self) -> TouchOutResetW<'_, TouchWorkSpec> {
        TouchOutResetW::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn touch_out_gate(&mut self) -> TouchOutGateW<'_, TouchWorkSpec> {
        TouchOutGateW::new(self, 27)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_work::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_work::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TouchWorkSpec;
impl crate::RegisterSpec for TouchWorkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_work::R`](R) reader structure"]
impl crate::Readable for TouchWorkSpec {}
#[doc = "`write(|w| ..)` method takes [`touch_work::W`](W) writer structure"]
impl crate::Writable for TouchWorkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOUCH_WORK to value 0"]
impl crate::Resettable for TouchWorkSpec {}
