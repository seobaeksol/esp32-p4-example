#[doc = "Register `CORE_1_DEBUG_MODE` reader"]
pub type R = crate::R<Core1DebugModeSpec>;
#[doc = "Field `CORE_1_DEBUG_MODE` reader - cpu debug mode status, 1 means cpu enter debug mode."]
pub type Core1DebugModeR = crate::BitReader;
#[doc = "Field `CORE_1_DEBUG_MODULE_ACTIVE` reader - cpu debug_module active status"]
pub type Core1DebugModuleActiveR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - cpu debug mode status, 1 means cpu enter debug mode."]
    #[inline(always)]
    pub fn core_1_debug_mode(&self) -> Core1DebugModeR {
        Core1DebugModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - cpu debug_module active status"]
    #[inline(always)]
    pub fn core_1_debug_module_active(&self) -> Core1DebugModuleActiveR {
        Core1DebugModuleActiveR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "cpu status register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_debug_mode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Core1DebugModeSpec;
impl crate::RegisterSpec for Core1DebugModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_debug_mode::R`](R) reader structure"]
impl crate::Readable for Core1DebugModeSpec {}
#[doc = "`reset()` method sets CORE_1_DEBUG_MODE to value 0"]
impl crate::Resettable for Core1DebugModeSpec {}
