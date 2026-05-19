#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<IntStSpec>;
#[doc = "Field `VDDBAT_CHARGE_UPVOLTAGE` reader - need_des"]
pub type VddbatChargeUpvoltageR = crate::BitReader;
#[doc = "Field `VDDBAT_CHARGE_UNDERVOLTAGE` reader - need_des"]
pub type VddbatChargeUndervoltageR = crate::BitReader;
#[doc = "Field `VDDBAT_UPVOLTAGE` reader - need_des"]
pub type VddbatUpvoltageR = crate::BitReader;
#[doc = "Field `VDDBAT_UNDERVOLTAGE` reader - need_des"]
pub type VddbatUndervoltageR = crate::BitReader;
#[doc = "Field `BOD_MODE0` reader - need_des"]
pub type BodMode0R = crate::BitReader;
impl R {
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn vddbat_charge_upvoltage(&self) -> VddbatChargeUpvoltageR {
        VddbatChargeUpvoltageR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn vddbat_charge_undervoltage(&self) -> VddbatChargeUndervoltageR {
        VddbatChargeUndervoltageR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn vddbat_upvoltage(&self) -> VddbatUpvoltageR {
        VddbatUpvoltageR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn vddbat_undervoltage(&self) -> VddbatUndervoltageR {
        VddbatUndervoltageR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn bod_mode0(&self) -> BodMode0R {
        BodMode0R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStSpec;
impl crate::RegisterSpec for IntStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for IntStSpec {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for IntStSpec {}
