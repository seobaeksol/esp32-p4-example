#[doc = "Register `PRESENT_STATE0` reader"]
pub type R = crate::R<PresentState0Spec>;
#[doc = "Field `SDA_LVL` reader - This bit is used to check the SCL line level to recover from error and for debugging. This bit reflects the value of synchronized scl_in_a."]
pub type SdaLvlR = crate::BitReader;
#[doc = "Field `SCL_LVL` reader - This bit is used to check the SDA line level to recover from error and for debugging. This bit reflects the value of synchronized sda_in_a."]
pub type SclLvlR = crate::BitReader;
#[doc = "Field `BUS_BUSY` reader - NA"]
pub type BusBusyR = crate::BitReader;
#[doc = "Field `BUS_FREE` reader - NA"]
pub type BusFreeR = crate::BitReader;
#[doc = "Field `CMD_TID` reader - NA"]
pub type CmdTidR = crate::FieldReader;
#[doc = "Field `SCL_GEN_FSM_STATE` reader - NA"]
pub type SclGenFsmStateR = crate::FieldReader;
#[doc = "Field `IBI_EV_HANDLE_FSM_STATE` reader - NA"]
pub type IbiEvHandleFsmStateR = crate::FieldReader;
#[doc = "Field `I2C_MODE_FSM_STATE` reader - NA"]
pub type I2cModeFsmStateR = crate::FieldReader;
#[doc = "Field `SDR_MODE_FSM_STATE` reader - NA"]
pub type SdrModeFsmStateR = crate::FieldReader;
#[doc = "Field `DAA_MODE_FSM_STATE` reader - Reflects whether the Master Controller is in IDLE or not. This bit will be set when all the buffer(Command, Response, IBI, Transmit, Receive) are empty along with the Master State machine is in idle state. 0X0: not in idle 0x1: in idle"]
pub type DaaModeFsmStateR = crate::FieldReader;
#[doc = "Field `MAIN_FSM_STATE` reader - NA"]
pub type MainFsmStateR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - This bit is used to check the SCL line level to recover from error and for debugging. This bit reflects the value of synchronized scl_in_a."]
    #[inline(always)]
    pub fn sda_lvl(&self) -> SdaLvlR {
        SdaLvlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is used to check the SDA line level to recover from error and for debugging. This bit reflects the value of synchronized sda_in_a."]
    #[inline(always)]
    pub fn scl_lvl(&self) -> SclLvlR {
        SclLvlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn bus_busy(&self) -> BusBusyR {
        BusBusyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn bus_free(&self) -> BusFreeR {
        BusFreeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 9:12 - NA"]
    #[inline(always)]
    pub fn cmd_tid(&self) -> CmdTidR {
        CmdTidR::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:15 - NA"]
    #[inline(always)]
    pub fn scl_gen_fsm_state(&self) -> SclGenFsmStateR {
        SclGenFsmStateR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:18 - NA"]
    #[inline(always)]
    pub fn ibi_ev_handle_fsm_state(&self) -> IbiEvHandleFsmStateR {
        IbiEvHandleFsmStateR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - NA"]
    #[inline(always)]
    pub fn i2c_mode_fsm_state(&self) -> I2cModeFsmStateR {
        I2cModeFsmStateR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:25 - NA"]
    #[inline(always)]
    pub fn sdr_mode_fsm_state(&self) -> SdrModeFsmStateR {
        SdrModeFsmStateR::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 26:28 - Reflects whether the Master Controller is in IDLE or not. This bit will be set when all the buffer(Command, Response, IBI, Transmit, Receive) are empty along with the Master State machine is in idle state. 0X0: not in idle 0x1: in idle"]
    #[inline(always)]
    pub fn daa_mode_fsm_state(&self) -> DaaModeFsmStateR {
        DaaModeFsmStateR::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:31 - NA"]
    #[inline(always)]
    pub fn main_fsm_state(&self) -> MainFsmStateR {
        MainFsmStateR::new(((self.bits >> 29) & 7) as u8)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`present_state0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PresentState0Spec;
impl crate::RegisterSpec for PresentState0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`present_state0::R`](R) reader structure"]
impl crate::Readable for PresentState0Spec {}
#[doc = "`reset()` method sets PRESENT_STATE0 to value 0x03"]
impl crate::Resettable for PresentState0Spec {
    const RESET_VALUE: u32 = 0x03;
}
