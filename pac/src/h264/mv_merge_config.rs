#[doc = "Register `MV_MERGE_CONFIG` reader"]
pub type R = crate::R<MvMergeConfigSpec>;
#[doc = "Register `MV_MERGE_CONFIG` writer"]
pub type W = crate::W<MvMergeConfigSpec>;
#[doc = "Field `MV_MERGE_TYPE` reader - Configure mv merge type.\\\\0: merge p16x16 mv\\\\1: merge min mv\\\\2: merge max mv\\\\3: not valid."]
pub type MvMergeTypeR = crate::FieldReader;
#[doc = "Field `MV_MERGE_TYPE` writer - Configure mv merge type.\\\\0: merge p16x16 mv\\\\1: merge min mv\\\\2: merge max mv\\\\3: not valid."]
pub type MvMergeTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INT_MV_OUT_EN` reader - Configure mv merge output integer part not zero mv or all part not zero mv.\\\\0: output all part not zero mv\\\\1: output integer part not zero mv."]
pub type IntMvOutEnR = crate::BitReader;
#[doc = "Field `INT_MV_OUT_EN` writer - Configure mv merge output integer part not zero mv or all part not zero mv.\\\\0: output all part not zero mv\\\\1: output integer part not zero mv."]
pub type IntMvOutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A_MV_MERGE_EN` reader - Configure whether or not to enable video A mv merge.\\\\0: disable\\\\1: enable."]
pub type AMvMergeEnR = crate::BitReader;
#[doc = "Field `A_MV_MERGE_EN` writer - Configure whether or not to enable video A mv merge.\\\\0: disable\\\\1: enable."]
pub type AMvMergeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B_MV_MERGE_EN` reader - Configure whether or not to enable video B mv merge.\\\\0: disable\\\\1: enable."]
pub type BMvMergeEnR = crate::BitReader;
#[doc = "Field `B_MV_MERGE_EN` writer - Configure whether or not to enable video B mv merge.\\\\0: disable\\\\1: enable."]
pub type BMvMergeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB_VALID_NUM` reader - Represents the valid mb number of mv merge output."]
pub type MbValidNumR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:1 - Configure mv merge type.\\\\0: merge p16x16 mv\\\\1: merge min mv\\\\2: merge max mv\\\\3: not valid."]
    #[inline(always)]
    pub fn mv_merge_type(&self) -> MvMergeTypeR {
        MvMergeTypeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Configure mv merge output integer part not zero mv or all part not zero mv.\\\\0: output all part not zero mv\\\\1: output integer part not zero mv."]
    #[inline(always)]
    pub fn int_mv_out_en(&self) -> IntMvOutEnR {
        IntMvOutEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configure whether or not to enable video A mv merge.\\\\0: disable\\\\1: enable."]
    #[inline(always)]
    pub fn a_mv_merge_en(&self) -> AMvMergeEnR {
        AMvMergeEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configure whether or not to enable video B mv merge.\\\\0: disable\\\\1: enable."]
    #[inline(always)]
    pub fn b_mv_merge_en(&self) -> BMvMergeEnR {
        BMvMergeEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:17 - Represents the valid mb number of mv merge output."]
    #[inline(always)]
    pub fn mb_valid_num(&self) -> MbValidNumR {
        MbValidNumR::new(((self.bits >> 5) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Configure mv merge type.\\\\0: merge p16x16 mv\\\\1: merge min mv\\\\2: merge max mv\\\\3: not valid."]
    #[inline(always)]
    pub fn mv_merge_type(&mut self) -> MvMergeTypeW<'_, MvMergeConfigSpec> {
        MvMergeTypeW::new(self, 0)
    }
    #[doc = "Bit 2 - Configure mv merge output integer part not zero mv or all part not zero mv.\\\\0: output all part not zero mv\\\\1: output integer part not zero mv."]
    #[inline(always)]
    pub fn int_mv_out_en(&mut self) -> IntMvOutEnW<'_, MvMergeConfigSpec> {
        IntMvOutEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Configure whether or not to enable video A mv merge.\\\\0: disable\\\\1: enable."]
    #[inline(always)]
    pub fn a_mv_merge_en(&mut self) -> AMvMergeEnW<'_, MvMergeConfigSpec> {
        AMvMergeEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Configure whether or not to enable video B mv merge.\\\\0: disable\\\\1: enable."]
    #[inline(always)]
    pub fn b_mv_merge_en(&mut self) -> BMvMergeEnW<'_, MvMergeConfigSpec> {
        BMvMergeEnW::new(self, 4)
    }
}
#[doc = "Mv merge configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mv_merge_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mv_merge_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MvMergeConfigSpec;
impl crate::RegisterSpec for MvMergeConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mv_merge_config::R`](R) reader structure"]
impl crate::Readable for MvMergeConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`mv_merge_config::W`](W) writer structure"]
impl crate::Writable for MvMergeConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MV_MERGE_CONFIG to value 0"]
impl crate::Resettable for MvMergeConfigSpec {}
