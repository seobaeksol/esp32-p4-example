#[doc = "Register `VDDBAT_CHARGE_CNTL` reader"]
pub type R = crate::R<VddbatChargeCntlSpec>;
#[doc = "Register `VDDBAT_CHARGE_CNTL` writer"]
pub type W = crate::W<VddbatChargeCntlSpec>;
#[doc = "Field `VDDBAT_CHARGE_UNDERVOLTAGE_FLAG` reader - need_des"]
pub type VddbatChargeUndervoltageFlagR = crate::BitReader;
#[doc = "Field `VDDBAT_CHARGE_CHARGER` reader - need_des"]
pub type VddbatChargeChargerR = crate::BitReader;
#[doc = "Field `VDDBAT_CHARGE_CHARGER` writer - need_des"]
pub type VddbatChargeChargerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDBAT_CHARGE_CNT_CLR` reader - need_des"]
pub type VddbatChargeCntClrR = crate::BitReader;
#[doc = "Field `VDDBAT_CHARGE_CNT_CLR` writer - need_des"]
pub type VddbatChargeCntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDBAT_CHARGE_UPVOLTAGE_TARGET` reader - need_des"]
pub type VddbatChargeUpvoltageTargetR = crate::FieldReader<u16>;
#[doc = "Field `VDDBAT_CHARGE_UPVOLTAGE_TARGET` writer - need_des"]
pub type VddbatChargeUpvoltageTargetW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `VDDBAT_CHARGE_UNDERVOLTAGE_TARGET` reader - need_des"]
pub type VddbatChargeUndervoltageTargetR = crate::FieldReader<u16>;
#[doc = "Field `VDDBAT_CHARGE_UNDERVOLTAGE_TARGET` writer - need_des"]
pub type VddbatChargeUndervoltageTargetW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn vddbat_charge_undervoltage_flag(&self) -> VddbatChargeUndervoltageFlagR {
        VddbatChargeUndervoltageFlagR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    pub fn vddbat_charge_charger(&self) -> VddbatChargeChargerR {
        VddbatChargeChargerR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    pub fn vddbat_charge_cnt_clr(&self) -> VddbatChargeCntClrR {
        VddbatChargeCntClrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:21 - need_des"]
    #[inline(always)]
    pub fn vddbat_charge_upvoltage_target(&self) -> VddbatChargeUpvoltageTargetR {
        VddbatChargeUpvoltageTargetR::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bits 22:31 - need_des"]
    #[inline(always)]
    pub fn vddbat_charge_undervoltage_target(&self) -> VddbatChargeUndervoltageTargetR {
        VddbatChargeUndervoltageTargetR::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    pub fn vddbat_charge_charger(&mut self) -> VddbatChargeChargerW<'_, VddbatChargeCntlSpec> {
        VddbatChargeChargerW::new(self, 10)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    pub fn vddbat_charge_cnt_clr(&mut self) -> VddbatChargeCntClrW<'_, VddbatChargeCntlSpec> {
        VddbatChargeCntClrW::new(self, 11)
    }
    #[doc = "Bits 12:21 - need_des"]
    #[inline(always)]
    pub fn vddbat_charge_upvoltage_target(
        &mut self,
    ) -> VddbatChargeUpvoltageTargetW<'_, VddbatChargeCntlSpec> {
        VddbatChargeUpvoltageTargetW::new(self, 12)
    }
    #[doc = "Bits 22:31 - need_des"]
    #[inline(always)]
    pub fn vddbat_charge_undervoltage_target(
        &mut self,
    ) -> VddbatChargeUndervoltageTargetW<'_, VddbatChargeCntlSpec> {
        VddbatChargeUndervoltageTargetW::new(self, 22)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`vddbat_charge_cntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vddbat_charge_cntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VddbatChargeCntlSpec;
impl crate::RegisterSpec for VddbatChargeCntlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vddbat_charge_cntl::R`](R) reader structure"]
impl crate::Readable for VddbatChargeCntlSpec {}
#[doc = "`write(|w| ..)` method takes [`vddbat_charge_cntl::W`](W) writer structure"]
impl crate::Writable for VddbatChargeCntlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDDBAT_CHARGE_CNTL to value 0xffc0_0000"]
impl crate::Resettable for VddbatChargeCntlSpec {
    const RESET_VALUE: u32 = 0xffc0_0000;
}
