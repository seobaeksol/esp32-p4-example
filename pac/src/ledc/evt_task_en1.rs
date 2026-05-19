#[doc = "Register `EVT_TASK_EN1` reader"]
pub type R = crate::R<EvtTaskEn1Spec>;
#[doc = "Register `EVT_TASK_EN1` writer"]
pub type W = crate::W<EvtTaskEn1Spec>;
#[doc = "Field `TASK_TIMER0_RES_UPDATE_EN` reader - Configures whether or not to enable ledc_timer0_res_update task.\\\\0: Disable\\\\1: Enable"]
pub type TaskTimer0ResUpdateEnR = crate::BitReader;
#[doc = "Field `TASK_TIMER0_RES_UPDATE_EN` writer - Configures whether or not to enable ledc_timer0_res_update task.\\\\0: Disable\\\\1: Enable"]
pub type TaskTimer0ResUpdateEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_TIMER1_RES_UPDATE_EN` reader - Configures whether or not to enable ledc_timer1_res_update task.\\\\0: Disable\\\\1: Enable"]
pub type TaskTimer1ResUpdateEnR = crate::BitReader;
#[doc = "Field `TASK_TIMER1_RES_UPDATE_EN` writer - Configures whether or not to enable ledc_timer1_res_update task.\\\\0: Disable\\\\1: Enable"]
pub type TaskTimer1ResUpdateEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_TIMER2_RES_UPDATE_EN` reader - Configures whether or not to enable ledc_timer2_res_update task.\\\\0: Disable\\\\1: Enable"]
pub type TaskTimer2ResUpdateEnR = crate::BitReader;
#[doc = "Field `TASK_TIMER2_RES_UPDATE_EN` writer - Configures whether or not to enable ledc_timer2_res_update task.\\\\0: Disable\\\\1: Enable"]
pub type TaskTimer2ResUpdateEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_TIMER3_RES_UPDATE_EN` reader - Configures whether or not to enable ledc_timer3_res_update task.\\\\0: Disable\\\\1: Enable"]
pub type TaskTimer3ResUpdateEnR = crate::BitReader;
#[doc = "Field `TASK_TIMER3_RES_UPDATE_EN` writer - Configures whether or not to enable ledc_timer3_res_update task.\\\\0: Disable\\\\1: Enable"]
pub type TaskTimer3ResUpdateEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_TIMER0_CAP_EN` reader - Configures whether or not to enable ledc_timer0_cap task.\\\\0: Disable\\\\1: Enable"]
pub type TaskTimer0CapEnR = crate::BitReader;
#[doc = "Field `TASK_TIMER0_CAP_EN` writer - Configures whether or not to enable ledc_timer0_cap task.\\\\0: Disable\\\\1: Enable"]
pub type TaskTimer0CapEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_TIMER1_CAP_EN` reader - Configures whether or not to enable ledc_timer1_cap task.\\\\0: Disable\\\\1: Enable"]
pub type TaskTimer1CapEnR = crate::BitReader;
#[doc = "Field `TASK_TIMER1_CAP_EN` writer - Configures whether or not to enable ledc_timer1_cap task.\\\\0: Disable\\\\1: Enable"]
pub type TaskTimer1CapEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_TIMER2_CAP_EN` reader - Configures whether or not to enable ledc_timer2_cap task.\\\\0: Disable\\\\1: Enable"]
pub type TaskTimer2CapEnR = crate::BitReader;
#[doc = "Field `TASK_TIMER2_CAP_EN` writer - Configures whether or not to enable ledc_timer2_cap task.\\\\0: Disable\\\\1: Enable"]
pub type TaskTimer2CapEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_TIMER3_CAP_EN` reader - Configures whether or not to enable ledc_timer3_cap task.\\\\0: Disable\\\\1: Enable"]
pub type TaskTimer3CapEnR = crate::BitReader;
#[doc = "Field `TASK_TIMER3_CAP_EN` writer - Configures whether or not to enable ledc_timer3_cap task.\\\\0: Disable\\\\1: Enable"]
pub type TaskTimer3CapEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_SIG_OUT_DIS_CH0_EN` reader - Configures whether or not to enable ledc_ch0_sig_out_dis task.\\\\0: Disable\\\\1: Enable"]
pub type TaskSigOutDisCh0EnR = crate::BitReader;
#[doc = "Field `TASK_SIG_OUT_DIS_CH0_EN` writer - Configures whether or not to enable ledc_ch0_sig_out_dis task.\\\\0: Disable\\\\1: Enable"]
pub type TaskSigOutDisCh0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_SIG_OUT_DIS_CH1_EN` reader - Configures whether or not to enable ledc_ch1_sig_out_dis task.\\\\0: Disable\\\\1: Enable"]
pub type TaskSigOutDisCh1EnR = crate::BitReader;
#[doc = "Field `TASK_SIG_OUT_DIS_CH1_EN` writer - Configures whether or not to enable ledc_ch1_sig_out_dis task.\\\\0: Disable\\\\1: Enable"]
pub type TaskSigOutDisCh1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_SIG_OUT_DIS_CH2_EN` reader - Configures whether or not to enable ledc_ch2_sig_out_dis task.\\\\0: Disable\\\\1: Enable"]
pub type TaskSigOutDisCh2EnR = crate::BitReader;
#[doc = "Field `TASK_SIG_OUT_DIS_CH2_EN` writer - Configures whether or not to enable ledc_ch2_sig_out_dis task.\\\\0: Disable\\\\1: Enable"]
pub type TaskSigOutDisCh2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_SIG_OUT_DIS_CH3_EN` reader - Configures whether or not to enable ledc_ch3_sig_out_dis task.\\\\0: Disable\\\\1: Enable"]
pub type TaskSigOutDisCh3EnR = crate::BitReader;
#[doc = "Field `TASK_SIG_OUT_DIS_CH3_EN` writer - Configures whether or not to enable ledc_ch3_sig_out_dis task.\\\\0: Disable\\\\1: Enable"]
pub type TaskSigOutDisCh3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_SIG_OUT_DIS_CH4_EN` reader - Configures whether or not to enable ledc_ch4_sig_out_dis task.\\\\0: Disable\\\\1: Enable"]
pub type TaskSigOutDisCh4EnR = crate::BitReader;
#[doc = "Field `TASK_SIG_OUT_DIS_CH4_EN` writer - Configures whether or not to enable ledc_ch4_sig_out_dis task.\\\\0: Disable\\\\1: Enable"]
pub type TaskSigOutDisCh4EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_SIG_OUT_DIS_CH5_EN` reader - Configures whether or not to enable ledc_ch5_sig_out_dis task.\\\\0: Disable\\\\1: Enable"]
pub type TaskSigOutDisCh5EnR = crate::BitReader;
#[doc = "Field `TASK_SIG_OUT_DIS_CH5_EN` writer - Configures whether or not to enable ledc_ch5_sig_out_dis task.\\\\0: Disable\\\\1: Enable"]
pub type TaskSigOutDisCh5EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_SIG_OUT_DIS_CH6_EN` reader - Configures whether or not to enable ledc_ch6_sig_out_dis task.\\\\0: Disable\\\\1: Enable"]
pub type TaskSigOutDisCh6EnR = crate::BitReader;
#[doc = "Field `TASK_SIG_OUT_DIS_CH6_EN` writer - Configures whether or not to enable ledc_ch6_sig_out_dis task.\\\\0: Disable\\\\1: Enable"]
pub type TaskSigOutDisCh6EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_SIG_OUT_DIS_CH7_EN` reader - Configures whether or not to enable ledc_ch7_sig_out_dis task.\\\\0: Disable\\\\1: Enable"]
pub type TaskSigOutDisCh7EnR = crate::BitReader;
#[doc = "Field `TASK_SIG_OUT_DIS_CH7_EN` writer - Configures whether or not to enable ledc_ch7_sig_out_dis task.\\\\0: Disable\\\\1: Enable"]
pub type TaskSigOutDisCh7EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_OVF_CNT_RST_CH0_EN` reader - Configures whether or not to enable ledc_ch0_ovf_cnt_rst task.\\\\0: Disable\\\\1: Enable"]
pub type TaskOvfCntRstCh0EnR = crate::BitReader;
#[doc = "Field `TASK_OVF_CNT_RST_CH0_EN` writer - Configures whether or not to enable ledc_ch0_ovf_cnt_rst task.\\\\0: Disable\\\\1: Enable"]
pub type TaskOvfCntRstCh0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_OVF_CNT_RST_CH1_EN` reader - Configures whether or not to enable ledc_ch1_ovf_cnt_rst task.\\\\0: Disable\\\\1: Enable"]
pub type TaskOvfCntRstCh1EnR = crate::BitReader;
#[doc = "Field `TASK_OVF_CNT_RST_CH1_EN` writer - Configures whether or not to enable ledc_ch1_ovf_cnt_rst task.\\\\0: Disable\\\\1: Enable"]
pub type TaskOvfCntRstCh1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_OVF_CNT_RST_CH2_EN` reader - Configures whether or not to enable ledc_ch2_ovf_cnt_rst task.\\\\0: Disable\\\\1: Enable"]
pub type TaskOvfCntRstCh2EnR = crate::BitReader;
#[doc = "Field `TASK_OVF_CNT_RST_CH2_EN` writer - Configures whether or not to enable ledc_ch2_ovf_cnt_rst task.\\\\0: Disable\\\\1: Enable"]
pub type TaskOvfCntRstCh2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_OVF_CNT_RST_CH3_EN` reader - Configures whether or not to enable ledc_ch3_ovf_cnt_rst task.\\\\0: Disable\\\\1: Enable"]
pub type TaskOvfCntRstCh3EnR = crate::BitReader;
#[doc = "Field `TASK_OVF_CNT_RST_CH3_EN` writer - Configures whether or not to enable ledc_ch3_ovf_cnt_rst task.\\\\0: Disable\\\\1: Enable"]
pub type TaskOvfCntRstCh3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_OVF_CNT_RST_CH4_EN` reader - Configures whether or not to enable ledc_ch4_ovf_cnt_rst task.\\\\0: Disable\\\\1: Enable"]
pub type TaskOvfCntRstCh4EnR = crate::BitReader;
#[doc = "Field `TASK_OVF_CNT_RST_CH4_EN` writer - Configures whether or not to enable ledc_ch4_ovf_cnt_rst task.\\\\0: Disable\\\\1: Enable"]
pub type TaskOvfCntRstCh4EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_OVF_CNT_RST_CH5_EN` reader - Configures whether or not to enable ledc_ch5_ovf_cnt_rst task.\\\\0: Disable\\\\1: Enable"]
pub type TaskOvfCntRstCh5EnR = crate::BitReader;
#[doc = "Field `TASK_OVF_CNT_RST_CH5_EN` writer - Configures whether or not to enable ledc_ch5_ovf_cnt_rst task.\\\\0: Disable\\\\1: Enable"]
pub type TaskOvfCntRstCh5EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_OVF_CNT_RST_CH6_EN` reader - Configures whether or not to enable ledc_ch6_ovf_cnt_rst task.\\\\0: Disable\\\\1: Enable"]
pub type TaskOvfCntRstCh6EnR = crate::BitReader;
#[doc = "Field `TASK_OVF_CNT_RST_CH6_EN` writer - Configures whether or not to enable ledc_ch6_ovf_cnt_rst task.\\\\0: Disable\\\\1: Enable"]
pub type TaskOvfCntRstCh6EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_OVF_CNT_RST_CH7_EN` reader - Configures whether or not to enable ledc_ch7_ovf_cnt_rst task.\\\\0: Disable\\\\1: Enable"]
pub type TaskOvfCntRstCh7EnR = crate::BitReader;
#[doc = "Field `TASK_OVF_CNT_RST_CH7_EN` writer - Configures whether or not to enable ledc_ch7_ovf_cnt_rst task.\\\\0: Disable\\\\1: Enable"]
pub type TaskOvfCntRstCh7EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_TIMER0_RST_EN` reader - Configures whether or not to enable ledc_timer0_rst task.\\\\0: Disable\\\\1: Enable"]
pub type TaskTimer0RstEnR = crate::BitReader;
#[doc = "Field `TASK_TIMER0_RST_EN` writer - Configures whether or not to enable ledc_timer0_rst task.\\\\0: Disable\\\\1: Enable"]
pub type TaskTimer0RstEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_TIMER1_RST_EN` reader - Configures whether or not to enable ledc_timer1_rst task.\\\\0: Disable\\\\1: Enable"]
pub type TaskTimer1RstEnR = crate::BitReader;
#[doc = "Field `TASK_TIMER1_RST_EN` writer - Configures whether or not to enable ledc_timer1_rst task.\\\\0: Disable\\\\1: Enable"]
pub type TaskTimer1RstEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_TIMER2_RST_EN` reader - Configures whether or not to enable ledc_timer2_rst task.\\\\0: Disable\\\\1: Enable"]
pub type TaskTimer2RstEnR = crate::BitReader;
#[doc = "Field `TASK_TIMER2_RST_EN` writer - Configures whether or not to enable ledc_timer2_rst task.\\\\0: Disable\\\\1: Enable"]
pub type TaskTimer2RstEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_TIMER3_RST_EN` reader - Configures whether or not to enable ledc_timer3_rst task.\\\\0: Disable\\\\1: Enable"]
pub type TaskTimer3RstEnR = crate::BitReader;
#[doc = "Field `TASK_TIMER3_RST_EN` writer - Configures whether or not to enable ledc_timer3_rst task.\\\\0: Disable\\\\1: Enable"]
pub type TaskTimer3RstEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_TIMER0_PAUSE_RESUME_EN` reader - Configures whether or not to enable ledc_timer0_pause_resume task.\\\\0: Disable\\\\1: Enable"]
pub type TaskTimer0PauseResumeEnR = crate::BitReader;
#[doc = "Field `TASK_TIMER0_PAUSE_RESUME_EN` writer - Configures whether or not to enable ledc_timer0_pause_resume task.\\\\0: Disable\\\\1: Enable"]
pub type TaskTimer0PauseResumeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_TIMER1_PAUSE_RESUME_EN` reader - Configures whether or not to enable ledc_timer1_pause_resume task.\\\\0: Disable\\\\1: Enable"]
pub type TaskTimer1PauseResumeEnR = crate::BitReader;
#[doc = "Field `TASK_TIMER1_PAUSE_RESUME_EN` writer - Configures whether or not to enable ledc_timer1_pause_resume task.\\\\0: Disable\\\\1: Enable"]
pub type TaskTimer1PauseResumeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_TIMER2_PAUSE_RESUME_EN` reader - Configures whether or not to enable ledc_timer2_pause_resume task.\\\\0: Disable\\\\1: Enable"]
pub type TaskTimer2PauseResumeEnR = crate::BitReader;
#[doc = "Field `TASK_TIMER2_PAUSE_RESUME_EN` writer - Configures whether or not to enable ledc_timer2_pause_resume task.\\\\0: Disable\\\\1: Enable"]
pub type TaskTimer2PauseResumeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_TIMER3_PAUSE_RESUME_EN` reader - Configures whether or not to enable ledc_timer3_pause_resume task.\\\\0: Disable\\\\1: Enable"]
pub type TaskTimer3PauseResumeEnR = crate::BitReader;
#[doc = "Field `TASK_TIMER3_PAUSE_RESUME_EN` writer - Configures whether or not to enable ledc_timer3_pause_resume task.\\\\0: Disable\\\\1: Enable"]
pub type TaskTimer3PauseResumeEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to enable ledc_timer0_res_update task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_timer0_res_update_en(&self) -> TaskTimer0ResUpdateEnR {
        TaskTimer0ResUpdateEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures whether or not to enable ledc_timer1_res_update task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_timer1_res_update_en(&self) -> TaskTimer1ResUpdateEnR {
        TaskTimer1ResUpdateEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures whether or not to enable ledc_timer2_res_update task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_timer2_res_update_en(&self) -> TaskTimer2ResUpdateEnR {
        TaskTimer2ResUpdateEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures whether or not to enable ledc_timer3_res_update task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_timer3_res_update_en(&self) -> TaskTimer3ResUpdateEnR {
        TaskTimer3ResUpdateEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures whether or not to enable ledc_timer0_cap task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_timer0_cap_en(&self) -> TaskTimer0CapEnR {
        TaskTimer0CapEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures whether or not to enable ledc_timer1_cap task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_timer1_cap_en(&self) -> TaskTimer1CapEnR {
        TaskTimer1CapEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures whether or not to enable ledc_timer2_cap task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_timer2_cap_en(&self) -> TaskTimer2CapEnR {
        TaskTimer2CapEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures whether or not to enable ledc_timer3_cap task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_timer3_cap_en(&self) -> TaskTimer3CapEnR {
        TaskTimer3CapEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Configures whether or not to enable ledc_ch0_sig_out_dis task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_sig_out_dis_ch0_en(&self) -> TaskSigOutDisCh0EnR {
        TaskSigOutDisCh0EnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Configures whether or not to enable ledc_ch1_sig_out_dis task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_sig_out_dis_ch1_en(&self) -> TaskSigOutDisCh1EnR {
        TaskSigOutDisCh1EnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Configures whether or not to enable ledc_ch2_sig_out_dis task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_sig_out_dis_ch2_en(&self) -> TaskSigOutDisCh2EnR {
        TaskSigOutDisCh2EnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Configures whether or not to enable ledc_ch3_sig_out_dis task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_sig_out_dis_ch3_en(&self) -> TaskSigOutDisCh3EnR {
        TaskSigOutDisCh3EnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Configures whether or not to enable ledc_ch4_sig_out_dis task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_sig_out_dis_ch4_en(&self) -> TaskSigOutDisCh4EnR {
        TaskSigOutDisCh4EnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Configures whether or not to enable ledc_ch5_sig_out_dis task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_sig_out_dis_ch5_en(&self) -> TaskSigOutDisCh5EnR {
        TaskSigOutDisCh5EnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Configures whether or not to enable ledc_ch6_sig_out_dis task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_sig_out_dis_ch6_en(&self) -> TaskSigOutDisCh6EnR {
        TaskSigOutDisCh6EnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Configures whether or not to enable ledc_ch7_sig_out_dis task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_sig_out_dis_ch7_en(&self) -> TaskSigOutDisCh7EnR {
        TaskSigOutDisCh7EnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Configures whether or not to enable ledc_ch0_ovf_cnt_rst task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch0_en(&self) -> TaskOvfCntRstCh0EnR {
        TaskOvfCntRstCh0EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Configures whether or not to enable ledc_ch1_ovf_cnt_rst task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch1_en(&self) -> TaskOvfCntRstCh1EnR {
        TaskOvfCntRstCh1EnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Configures whether or not to enable ledc_ch2_ovf_cnt_rst task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch2_en(&self) -> TaskOvfCntRstCh2EnR {
        TaskOvfCntRstCh2EnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Configures whether or not to enable ledc_ch3_ovf_cnt_rst task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch3_en(&self) -> TaskOvfCntRstCh3EnR {
        TaskOvfCntRstCh3EnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Configures whether or not to enable ledc_ch4_ovf_cnt_rst task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch4_en(&self) -> TaskOvfCntRstCh4EnR {
        TaskOvfCntRstCh4EnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Configures whether or not to enable ledc_ch5_ovf_cnt_rst task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch5_en(&self) -> TaskOvfCntRstCh5EnR {
        TaskOvfCntRstCh5EnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Configures whether or not to enable ledc_ch6_ovf_cnt_rst task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch6_en(&self) -> TaskOvfCntRstCh6EnR {
        TaskOvfCntRstCh6EnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Configures whether or not to enable ledc_ch7_ovf_cnt_rst task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch7_en(&self) -> TaskOvfCntRstCh7EnR {
        TaskOvfCntRstCh7EnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Configures whether or not to enable ledc_timer0_rst task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_timer0_rst_en(&self) -> TaskTimer0RstEnR {
        TaskTimer0RstEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Configures whether or not to enable ledc_timer1_rst task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_timer1_rst_en(&self) -> TaskTimer1RstEnR {
        TaskTimer1RstEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Configures whether or not to enable ledc_timer2_rst task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_timer2_rst_en(&self) -> TaskTimer2RstEnR {
        TaskTimer2RstEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Configures whether or not to enable ledc_timer3_rst task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_timer3_rst_en(&self) -> TaskTimer3RstEnR {
        TaskTimer3RstEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Configures whether or not to enable ledc_timer0_pause_resume task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_timer0_pause_resume_en(&self) -> TaskTimer0PauseResumeEnR {
        TaskTimer0PauseResumeEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Configures whether or not to enable ledc_timer1_pause_resume task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_timer1_pause_resume_en(&self) -> TaskTimer1PauseResumeEnR {
        TaskTimer1PauseResumeEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Configures whether or not to enable ledc_timer2_pause_resume task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_timer2_pause_resume_en(&self) -> TaskTimer2PauseResumeEnR {
        TaskTimer2PauseResumeEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Configures whether or not to enable ledc_timer3_pause_resume task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_timer3_pause_resume_en(&self) -> TaskTimer3PauseResumeEnR {
        TaskTimer3PauseResumeEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to enable ledc_timer0_res_update task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_timer0_res_update_en(&mut self) -> TaskTimer0ResUpdateEnW<'_, EvtTaskEn1Spec> {
        TaskTimer0ResUpdateEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to enable ledc_timer1_res_update task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_timer1_res_update_en(&mut self) -> TaskTimer1ResUpdateEnW<'_, EvtTaskEn1Spec> {
        TaskTimer1ResUpdateEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to enable ledc_timer2_res_update task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_timer2_res_update_en(&mut self) -> TaskTimer2ResUpdateEnW<'_, EvtTaskEn1Spec> {
        TaskTimer2ResUpdateEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to enable ledc_timer3_res_update task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_timer3_res_update_en(&mut self) -> TaskTimer3ResUpdateEnW<'_, EvtTaskEn1Spec> {
        TaskTimer3ResUpdateEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to enable ledc_timer0_cap task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_timer0_cap_en(&mut self) -> TaskTimer0CapEnW<'_, EvtTaskEn1Spec> {
        TaskTimer0CapEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether or not to enable ledc_timer1_cap task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_timer1_cap_en(&mut self) -> TaskTimer1CapEnW<'_, EvtTaskEn1Spec> {
        TaskTimer1CapEnW::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether or not to enable ledc_timer2_cap task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_timer2_cap_en(&mut self) -> TaskTimer2CapEnW<'_, EvtTaskEn1Spec> {
        TaskTimer2CapEnW::new(self, 6)
    }
    #[doc = "Bit 7 - Configures whether or not to enable ledc_timer3_cap task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_timer3_cap_en(&mut self) -> TaskTimer3CapEnW<'_, EvtTaskEn1Spec> {
        TaskTimer3CapEnW::new(self, 7)
    }
    #[doc = "Bit 8 - Configures whether or not to enable ledc_ch0_sig_out_dis task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_sig_out_dis_ch0_en(&mut self) -> TaskSigOutDisCh0EnW<'_, EvtTaskEn1Spec> {
        TaskSigOutDisCh0EnW::new(self, 8)
    }
    #[doc = "Bit 9 - Configures whether or not to enable ledc_ch1_sig_out_dis task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_sig_out_dis_ch1_en(&mut self) -> TaskSigOutDisCh1EnW<'_, EvtTaskEn1Spec> {
        TaskSigOutDisCh1EnW::new(self, 9)
    }
    #[doc = "Bit 10 - Configures whether or not to enable ledc_ch2_sig_out_dis task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_sig_out_dis_ch2_en(&mut self) -> TaskSigOutDisCh2EnW<'_, EvtTaskEn1Spec> {
        TaskSigOutDisCh2EnW::new(self, 10)
    }
    #[doc = "Bit 11 - Configures whether or not to enable ledc_ch3_sig_out_dis task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_sig_out_dis_ch3_en(&mut self) -> TaskSigOutDisCh3EnW<'_, EvtTaskEn1Spec> {
        TaskSigOutDisCh3EnW::new(self, 11)
    }
    #[doc = "Bit 12 - Configures whether or not to enable ledc_ch4_sig_out_dis task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_sig_out_dis_ch4_en(&mut self) -> TaskSigOutDisCh4EnW<'_, EvtTaskEn1Spec> {
        TaskSigOutDisCh4EnW::new(self, 12)
    }
    #[doc = "Bit 13 - Configures whether or not to enable ledc_ch5_sig_out_dis task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_sig_out_dis_ch5_en(&mut self) -> TaskSigOutDisCh5EnW<'_, EvtTaskEn1Spec> {
        TaskSigOutDisCh5EnW::new(self, 13)
    }
    #[doc = "Bit 14 - Configures whether or not to enable ledc_ch6_sig_out_dis task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_sig_out_dis_ch6_en(&mut self) -> TaskSigOutDisCh6EnW<'_, EvtTaskEn1Spec> {
        TaskSigOutDisCh6EnW::new(self, 14)
    }
    #[doc = "Bit 15 - Configures whether or not to enable ledc_ch7_sig_out_dis task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_sig_out_dis_ch7_en(&mut self) -> TaskSigOutDisCh7EnW<'_, EvtTaskEn1Spec> {
        TaskSigOutDisCh7EnW::new(self, 15)
    }
    #[doc = "Bit 16 - Configures whether or not to enable ledc_ch0_ovf_cnt_rst task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch0_en(&mut self) -> TaskOvfCntRstCh0EnW<'_, EvtTaskEn1Spec> {
        TaskOvfCntRstCh0EnW::new(self, 16)
    }
    #[doc = "Bit 17 - Configures whether or not to enable ledc_ch1_ovf_cnt_rst task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch1_en(&mut self) -> TaskOvfCntRstCh1EnW<'_, EvtTaskEn1Spec> {
        TaskOvfCntRstCh1EnW::new(self, 17)
    }
    #[doc = "Bit 18 - Configures whether or not to enable ledc_ch2_ovf_cnt_rst task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch2_en(&mut self) -> TaskOvfCntRstCh2EnW<'_, EvtTaskEn1Spec> {
        TaskOvfCntRstCh2EnW::new(self, 18)
    }
    #[doc = "Bit 19 - Configures whether or not to enable ledc_ch3_ovf_cnt_rst task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch3_en(&mut self) -> TaskOvfCntRstCh3EnW<'_, EvtTaskEn1Spec> {
        TaskOvfCntRstCh3EnW::new(self, 19)
    }
    #[doc = "Bit 20 - Configures whether or not to enable ledc_ch4_ovf_cnt_rst task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch4_en(&mut self) -> TaskOvfCntRstCh4EnW<'_, EvtTaskEn1Spec> {
        TaskOvfCntRstCh4EnW::new(self, 20)
    }
    #[doc = "Bit 21 - Configures whether or not to enable ledc_ch5_ovf_cnt_rst task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch5_en(&mut self) -> TaskOvfCntRstCh5EnW<'_, EvtTaskEn1Spec> {
        TaskOvfCntRstCh5EnW::new(self, 21)
    }
    #[doc = "Bit 22 - Configures whether or not to enable ledc_ch6_ovf_cnt_rst task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch6_en(&mut self) -> TaskOvfCntRstCh6EnW<'_, EvtTaskEn1Spec> {
        TaskOvfCntRstCh6EnW::new(self, 22)
    }
    #[doc = "Bit 23 - Configures whether or not to enable ledc_ch7_ovf_cnt_rst task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch7_en(&mut self) -> TaskOvfCntRstCh7EnW<'_, EvtTaskEn1Spec> {
        TaskOvfCntRstCh7EnW::new(self, 23)
    }
    #[doc = "Bit 24 - Configures whether or not to enable ledc_timer0_rst task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_timer0_rst_en(&mut self) -> TaskTimer0RstEnW<'_, EvtTaskEn1Spec> {
        TaskTimer0RstEnW::new(self, 24)
    }
    #[doc = "Bit 25 - Configures whether or not to enable ledc_timer1_rst task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_timer1_rst_en(&mut self) -> TaskTimer1RstEnW<'_, EvtTaskEn1Spec> {
        TaskTimer1RstEnW::new(self, 25)
    }
    #[doc = "Bit 26 - Configures whether or not to enable ledc_timer2_rst task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_timer2_rst_en(&mut self) -> TaskTimer2RstEnW<'_, EvtTaskEn1Spec> {
        TaskTimer2RstEnW::new(self, 26)
    }
    #[doc = "Bit 27 - Configures whether or not to enable ledc_timer3_rst task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_timer3_rst_en(&mut self) -> TaskTimer3RstEnW<'_, EvtTaskEn1Spec> {
        TaskTimer3RstEnW::new(self, 27)
    }
    #[doc = "Bit 28 - Configures whether or not to enable ledc_timer0_pause_resume task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_timer0_pause_resume_en(&mut self) -> TaskTimer0PauseResumeEnW<'_, EvtTaskEn1Spec> {
        TaskTimer0PauseResumeEnW::new(self, 28)
    }
    #[doc = "Bit 29 - Configures whether or not to enable ledc_timer1_pause_resume task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_timer1_pause_resume_en(&mut self) -> TaskTimer1PauseResumeEnW<'_, EvtTaskEn1Spec> {
        TaskTimer1PauseResumeEnW::new(self, 29)
    }
    #[doc = "Bit 30 - Configures whether or not to enable ledc_timer2_pause_resume task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_timer2_pause_resume_en(&mut self) -> TaskTimer2PauseResumeEnW<'_, EvtTaskEn1Spec> {
        TaskTimer2PauseResumeEnW::new(self, 30)
    }
    #[doc = "Bit 31 - Configures whether or not to enable ledc_timer3_pause_resume task.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn task_timer3_pause_resume_en(&mut self) -> TaskTimer3PauseResumeEnW<'_, EvtTaskEn1Spec> {
        TaskTimer3PauseResumeEnW::new(self, 31)
    }
}
#[doc = "Ledc event task enable bit register1.\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_task_en1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_task_en1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvtTaskEn1Spec;
impl crate::RegisterSpec for EvtTaskEn1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evt_task_en1::R`](R) reader structure"]
impl crate::Readable for EvtTaskEn1Spec {}
#[doc = "`write(|w| ..)` method takes [`evt_task_en1::W`](W) writer structure"]
impl crate::Writable for EvtTaskEn1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVT_TASK_EN1 to value 0"]
impl crate::Resettable for EvtTaskEn1Spec {}
