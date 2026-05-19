#[doc = "Register `TASK_ST2` reader"]
pub type R = crate::R<TaskSt2Spec>;
#[doc = "Register `TASK_ST2` writer"]
pub type W = crate::W<TaskSt2Spec>;
#[doc = "Field `LEDC_TASK_TIMER0_PAUSE_ST` reader - Represents LEDC_task_timer0_pause trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskTimer0PauseStR = crate::BitReader;
#[doc = "Field `LEDC_TASK_TIMER0_PAUSE_ST` writer - Represents LEDC_task_timer0_pause trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskTimer0PauseStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC_TASK_TIMER1_PAUSE_ST` reader - Represents LEDC_task_timer1_pause trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskTimer1PauseStR = crate::BitReader;
#[doc = "Field `LEDC_TASK_TIMER1_PAUSE_ST` writer - Represents LEDC_task_timer1_pause trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskTimer1PauseStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC_TASK_TIMER2_PAUSE_ST` reader - Represents LEDC_task_timer2_pause trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskTimer2PauseStR = crate::BitReader;
#[doc = "Field `LEDC_TASK_TIMER2_PAUSE_ST` writer - Represents LEDC_task_timer2_pause trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskTimer2PauseStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC_TASK_TIMER3_PAUSE_ST` reader - Represents LEDC_task_timer3_pause trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskTimer3PauseStR = crate::BitReader;
#[doc = "Field `LEDC_TASK_TIMER3_PAUSE_ST` writer - Represents LEDC_task_timer3_pause trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskTimer3PauseStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC_TASK_GAMMA_RESTART_CH0_ST` reader - Represents LEDC_task_gamma_restart_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaRestartCh0StR = crate::BitReader;
#[doc = "Field `LEDC_TASK_GAMMA_RESTART_CH0_ST` writer - Represents LEDC_task_gamma_restart_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaRestartCh0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC_TASK_GAMMA_RESTART_CH1_ST` reader - Represents LEDC_task_gamma_restart_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaRestartCh1StR = crate::BitReader;
#[doc = "Field `LEDC_TASK_GAMMA_RESTART_CH1_ST` writer - Represents LEDC_task_gamma_restart_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaRestartCh1StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC_TASK_GAMMA_RESTART_CH2_ST` reader - Represents LEDC_task_gamma_restart_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaRestartCh2StR = crate::BitReader;
#[doc = "Field `LEDC_TASK_GAMMA_RESTART_CH2_ST` writer - Represents LEDC_task_gamma_restart_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaRestartCh2StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC_TASK_GAMMA_RESTART_CH3_ST` reader - Represents LEDC_task_gamma_restart_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaRestartCh3StR = crate::BitReader;
#[doc = "Field `LEDC_TASK_GAMMA_RESTART_CH3_ST` writer - Represents LEDC_task_gamma_restart_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaRestartCh3StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC_TASK_GAMMA_RESTART_CH4_ST` reader - Represents LEDC_task_gamma_restart_ch4 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaRestartCh4StR = crate::BitReader;
#[doc = "Field `LEDC_TASK_GAMMA_RESTART_CH4_ST` writer - Represents LEDC_task_gamma_restart_ch4 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaRestartCh4StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC_TASK_GAMMA_RESTART_CH5_ST` reader - Represents LEDC_task_gamma_restart_ch5 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaRestartCh5StR = crate::BitReader;
#[doc = "Field `LEDC_TASK_GAMMA_RESTART_CH5_ST` writer - Represents LEDC_task_gamma_restart_ch5 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaRestartCh5StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC_TASK_GAMMA_RESTART_CH6_ST` reader - Represents LEDC_task_gamma_restart_ch6 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaRestartCh6StR = crate::BitReader;
#[doc = "Field `LEDC_TASK_GAMMA_RESTART_CH6_ST` writer - Represents LEDC_task_gamma_restart_ch6 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaRestartCh6StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC_TASK_GAMMA_RESTART_CH7_ST` reader - Represents LEDC_task_gamma_restart_ch7 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaRestartCh7StR = crate::BitReader;
#[doc = "Field `LEDC_TASK_GAMMA_RESTART_CH7_ST` writer - Represents LEDC_task_gamma_restart_ch7 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaRestartCh7StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC_TASK_GAMMA_PAUSE_CH0_ST` reader - Represents LEDC_task_gamma_pause_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaPauseCh0StR = crate::BitReader;
#[doc = "Field `LEDC_TASK_GAMMA_PAUSE_CH0_ST` writer - Represents LEDC_task_gamma_pause_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaPauseCh0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC_TASK_GAMMA_PAUSE_CH1_ST` reader - Represents LEDC_task_gamma_pause_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaPauseCh1StR = crate::BitReader;
#[doc = "Field `LEDC_TASK_GAMMA_PAUSE_CH1_ST` writer - Represents LEDC_task_gamma_pause_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaPauseCh1StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC_TASK_GAMMA_PAUSE_CH2_ST` reader - Represents LEDC_task_gamma_pause_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaPauseCh2StR = crate::BitReader;
#[doc = "Field `LEDC_TASK_GAMMA_PAUSE_CH2_ST` writer - Represents LEDC_task_gamma_pause_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaPauseCh2StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC_TASK_GAMMA_PAUSE_CH3_ST` reader - Represents LEDC_task_gamma_pause_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaPauseCh3StR = crate::BitReader;
#[doc = "Field `LEDC_TASK_GAMMA_PAUSE_CH3_ST` writer - Represents LEDC_task_gamma_pause_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaPauseCh3StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC_TASK_GAMMA_PAUSE_CH4_ST` reader - Represents LEDC_task_gamma_pause_ch4 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaPauseCh4StR = crate::BitReader;
#[doc = "Field `LEDC_TASK_GAMMA_PAUSE_CH4_ST` writer - Represents LEDC_task_gamma_pause_ch4 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaPauseCh4StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC_TASK_GAMMA_PAUSE_CH5_ST` reader - Represents LEDC_task_gamma_pause_ch5 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaPauseCh5StR = crate::BitReader;
#[doc = "Field `LEDC_TASK_GAMMA_PAUSE_CH5_ST` writer - Represents LEDC_task_gamma_pause_ch5 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaPauseCh5StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC_TASK_GAMMA_PAUSE_CH6_ST` reader - Represents LEDC_task_gamma_pause_ch6 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaPauseCh6StR = crate::BitReader;
#[doc = "Field `LEDC_TASK_GAMMA_PAUSE_CH6_ST` writer - Represents LEDC_task_gamma_pause_ch6 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaPauseCh6StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC_TASK_GAMMA_PAUSE_CH7_ST` reader - Represents LEDC_task_gamma_pause_ch7 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaPauseCh7StR = crate::BitReader;
#[doc = "Field `LEDC_TASK_GAMMA_PAUSE_CH7_ST` writer - Represents LEDC_task_gamma_pause_ch7 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaPauseCh7StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC_TASK_GAMMA_RESUME_CH0_ST` reader - Represents LEDC_task_gamma_resume_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaResumeCh0StR = crate::BitReader;
#[doc = "Field `LEDC_TASK_GAMMA_RESUME_CH0_ST` writer - Represents LEDC_task_gamma_resume_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaResumeCh0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC_TASK_GAMMA_RESUME_CH1_ST` reader - Represents LEDC_task_gamma_resume_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaResumeCh1StR = crate::BitReader;
#[doc = "Field `LEDC_TASK_GAMMA_RESUME_CH1_ST` writer - Represents LEDC_task_gamma_resume_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaResumeCh1StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC_TASK_GAMMA_RESUME_CH2_ST` reader - Represents LEDC_task_gamma_resume_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaResumeCh2StR = crate::BitReader;
#[doc = "Field `LEDC_TASK_GAMMA_RESUME_CH2_ST` writer - Represents LEDC_task_gamma_resume_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaResumeCh2StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC_TASK_GAMMA_RESUME_CH3_ST` reader - Represents LEDC_task_gamma_resume_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaResumeCh3StR = crate::BitReader;
#[doc = "Field `LEDC_TASK_GAMMA_RESUME_CH3_ST` writer - Represents LEDC_task_gamma_resume_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaResumeCh3StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC_TASK_GAMMA_RESUME_CH4_ST` reader - Represents LEDC_task_gamma_resume_ch4 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaResumeCh4StR = crate::BitReader;
#[doc = "Field `LEDC_TASK_GAMMA_RESUME_CH4_ST` writer - Represents LEDC_task_gamma_resume_ch4 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaResumeCh4StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC_TASK_GAMMA_RESUME_CH5_ST` reader - Represents LEDC_task_gamma_resume_ch5 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaResumeCh5StR = crate::BitReader;
#[doc = "Field `LEDC_TASK_GAMMA_RESUME_CH5_ST` writer - Represents LEDC_task_gamma_resume_ch5 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaResumeCh5StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC_TASK_GAMMA_RESUME_CH6_ST` reader - Represents LEDC_task_gamma_resume_ch6 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaResumeCh6StR = crate::BitReader;
#[doc = "Field `LEDC_TASK_GAMMA_RESUME_CH6_ST` writer - Represents LEDC_task_gamma_resume_ch6 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaResumeCh6StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC_TASK_GAMMA_RESUME_CH7_ST` reader - Represents LEDC_task_gamma_resume_ch7 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaResumeCh7StR = crate::BitReader;
#[doc = "Field `LEDC_TASK_GAMMA_RESUME_CH7_ST` writer - Represents LEDC_task_gamma_resume_ch7 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type LedcTaskGammaResumeCh7StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG0_TASK_CNT_START_TIMER0_ST` reader - Represents TG0_task_cnt_start_timer0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Tg0TaskCntStartTimer0StR = crate::BitReader;
#[doc = "Field `TG0_TASK_CNT_START_TIMER0_ST` writer - Represents TG0_task_cnt_start_timer0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Tg0TaskCntStartTimer0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG0_TASK_ALARM_START_TIMER0_ST` reader - Represents TG0_task_alarm_start_timer0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Tg0TaskAlarmStartTimer0StR = crate::BitReader;
#[doc = "Field `TG0_TASK_ALARM_START_TIMER0_ST` writer - Represents TG0_task_alarm_start_timer0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Tg0TaskAlarmStartTimer0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG0_TASK_CNT_STOP_TIMER0_ST` reader - Represents TG0_task_cnt_stop_timer0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Tg0TaskCntStopTimer0StR = crate::BitReader;
#[doc = "Field `TG0_TASK_CNT_STOP_TIMER0_ST` writer - Represents TG0_task_cnt_stop_timer0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Tg0TaskCntStopTimer0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG0_TASK_CNT_RELOAD_TIMER0_ST` reader - Represents TG0_task_cnt_reload_timer0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Tg0TaskCntReloadTimer0StR = crate::BitReader;
#[doc = "Field `TG0_TASK_CNT_RELOAD_TIMER0_ST` writer - Represents TG0_task_cnt_reload_timer0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Tg0TaskCntReloadTimer0StW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Represents LEDC_task_timer0_pause trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_timer0_pause_st(&self) -> LedcTaskTimer0PauseStR {
        LedcTaskTimer0PauseStR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents LEDC_task_timer1_pause trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_timer1_pause_st(&self) -> LedcTaskTimer1PauseStR {
        LedcTaskTimer1PauseStR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Represents LEDC_task_timer2_pause trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_timer2_pause_st(&self) -> LedcTaskTimer2PauseStR {
        LedcTaskTimer2PauseStR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Represents LEDC_task_timer3_pause trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_timer3_pause_st(&self) -> LedcTaskTimer3PauseStR {
        LedcTaskTimer3PauseStR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents LEDC_task_gamma_restart_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_restart_ch0_st(&self) -> LedcTaskGammaRestartCh0StR {
        LedcTaskGammaRestartCh0StR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Represents LEDC_task_gamma_restart_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_restart_ch1_st(&self) -> LedcTaskGammaRestartCh1StR {
        LedcTaskGammaRestartCh1StR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Represents LEDC_task_gamma_restart_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_restart_ch2_st(&self) -> LedcTaskGammaRestartCh2StR {
        LedcTaskGammaRestartCh2StR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Represents LEDC_task_gamma_restart_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_restart_ch3_st(&self) -> LedcTaskGammaRestartCh3StR {
        LedcTaskGammaRestartCh3StR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Represents LEDC_task_gamma_restart_ch4 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_restart_ch4_st(&self) -> LedcTaskGammaRestartCh4StR {
        LedcTaskGammaRestartCh4StR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Represents LEDC_task_gamma_restart_ch5 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_restart_ch5_st(&self) -> LedcTaskGammaRestartCh5StR {
        LedcTaskGammaRestartCh5StR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Represents LEDC_task_gamma_restart_ch6 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_restart_ch6_st(&self) -> LedcTaskGammaRestartCh6StR {
        LedcTaskGammaRestartCh6StR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Represents LEDC_task_gamma_restart_ch7 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_restart_ch7_st(&self) -> LedcTaskGammaRestartCh7StR {
        LedcTaskGammaRestartCh7StR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Represents LEDC_task_gamma_pause_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_pause_ch0_st(&self) -> LedcTaskGammaPauseCh0StR {
        LedcTaskGammaPauseCh0StR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Represents LEDC_task_gamma_pause_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_pause_ch1_st(&self) -> LedcTaskGammaPauseCh1StR {
        LedcTaskGammaPauseCh1StR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Represents LEDC_task_gamma_pause_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_pause_ch2_st(&self) -> LedcTaskGammaPauseCh2StR {
        LedcTaskGammaPauseCh2StR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Represents LEDC_task_gamma_pause_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_pause_ch3_st(&self) -> LedcTaskGammaPauseCh3StR {
        LedcTaskGammaPauseCh3StR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Represents LEDC_task_gamma_pause_ch4 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_pause_ch4_st(&self) -> LedcTaskGammaPauseCh4StR {
        LedcTaskGammaPauseCh4StR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Represents LEDC_task_gamma_pause_ch5 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_pause_ch5_st(&self) -> LedcTaskGammaPauseCh5StR {
        LedcTaskGammaPauseCh5StR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Represents LEDC_task_gamma_pause_ch6 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_pause_ch6_st(&self) -> LedcTaskGammaPauseCh6StR {
        LedcTaskGammaPauseCh6StR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Represents LEDC_task_gamma_pause_ch7 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_pause_ch7_st(&self) -> LedcTaskGammaPauseCh7StR {
        LedcTaskGammaPauseCh7StR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Represents LEDC_task_gamma_resume_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_resume_ch0_st(&self) -> LedcTaskGammaResumeCh0StR {
        LedcTaskGammaResumeCh0StR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Represents LEDC_task_gamma_resume_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_resume_ch1_st(&self) -> LedcTaskGammaResumeCh1StR {
        LedcTaskGammaResumeCh1StR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Represents LEDC_task_gamma_resume_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_resume_ch2_st(&self) -> LedcTaskGammaResumeCh2StR {
        LedcTaskGammaResumeCh2StR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Represents LEDC_task_gamma_resume_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_resume_ch3_st(&self) -> LedcTaskGammaResumeCh3StR {
        LedcTaskGammaResumeCh3StR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Represents LEDC_task_gamma_resume_ch4 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_resume_ch4_st(&self) -> LedcTaskGammaResumeCh4StR {
        LedcTaskGammaResumeCh4StR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Represents LEDC_task_gamma_resume_ch5 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_resume_ch5_st(&self) -> LedcTaskGammaResumeCh5StR {
        LedcTaskGammaResumeCh5StR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Represents LEDC_task_gamma_resume_ch6 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_resume_ch6_st(&self) -> LedcTaskGammaResumeCh6StR {
        LedcTaskGammaResumeCh6StR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Represents LEDC_task_gamma_resume_ch7 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_resume_ch7_st(&self) -> LedcTaskGammaResumeCh7StR {
        LedcTaskGammaResumeCh7StR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Represents TG0_task_cnt_start_timer0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn tg0_task_cnt_start_timer0_st(&self) -> Tg0TaskCntStartTimer0StR {
        Tg0TaskCntStartTimer0StR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Represents TG0_task_alarm_start_timer0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn tg0_task_alarm_start_timer0_st(&self) -> Tg0TaskAlarmStartTimer0StR {
        Tg0TaskAlarmStartTimer0StR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Represents TG0_task_cnt_stop_timer0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn tg0_task_cnt_stop_timer0_st(&self) -> Tg0TaskCntStopTimer0StR {
        Tg0TaskCntStopTimer0StR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Represents TG0_task_cnt_reload_timer0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn tg0_task_cnt_reload_timer0_st(&self) -> Tg0TaskCntReloadTimer0StR {
        Tg0TaskCntReloadTimer0StR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Represents LEDC_task_timer0_pause trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_timer0_pause_st(&mut self) -> LedcTaskTimer0PauseStW<'_, TaskSt2Spec> {
        LedcTaskTimer0PauseStW::new(self, 0)
    }
    #[doc = "Bit 1 - Represents LEDC_task_timer1_pause trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_timer1_pause_st(&mut self) -> LedcTaskTimer1PauseStW<'_, TaskSt2Spec> {
        LedcTaskTimer1PauseStW::new(self, 1)
    }
    #[doc = "Bit 2 - Represents LEDC_task_timer2_pause trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_timer2_pause_st(&mut self) -> LedcTaskTimer2PauseStW<'_, TaskSt2Spec> {
        LedcTaskTimer2PauseStW::new(self, 2)
    }
    #[doc = "Bit 3 - Represents LEDC_task_timer3_pause trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_timer3_pause_st(&mut self) -> LedcTaskTimer3PauseStW<'_, TaskSt2Spec> {
        LedcTaskTimer3PauseStW::new(self, 3)
    }
    #[doc = "Bit 4 - Represents LEDC_task_gamma_restart_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_restart_ch0_st(
        &mut self,
    ) -> LedcTaskGammaRestartCh0StW<'_, TaskSt2Spec> {
        LedcTaskGammaRestartCh0StW::new(self, 4)
    }
    #[doc = "Bit 5 - Represents LEDC_task_gamma_restart_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_restart_ch1_st(
        &mut self,
    ) -> LedcTaskGammaRestartCh1StW<'_, TaskSt2Spec> {
        LedcTaskGammaRestartCh1StW::new(self, 5)
    }
    #[doc = "Bit 6 - Represents LEDC_task_gamma_restart_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_restart_ch2_st(
        &mut self,
    ) -> LedcTaskGammaRestartCh2StW<'_, TaskSt2Spec> {
        LedcTaskGammaRestartCh2StW::new(self, 6)
    }
    #[doc = "Bit 7 - Represents LEDC_task_gamma_restart_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_restart_ch3_st(
        &mut self,
    ) -> LedcTaskGammaRestartCh3StW<'_, TaskSt2Spec> {
        LedcTaskGammaRestartCh3StW::new(self, 7)
    }
    #[doc = "Bit 8 - Represents LEDC_task_gamma_restart_ch4 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_restart_ch4_st(
        &mut self,
    ) -> LedcTaskGammaRestartCh4StW<'_, TaskSt2Spec> {
        LedcTaskGammaRestartCh4StW::new(self, 8)
    }
    #[doc = "Bit 9 - Represents LEDC_task_gamma_restart_ch5 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_restart_ch5_st(
        &mut self,
    ) -> LedcTaskGammaRestartCh5StW<'_, TaskSt2Spec> {
        LedcTaskGammaRestartCh5StW::new(self, 9)
    }
    #[doc = "Bit 10 - Represents LEDC_task_gamma_restart_ch6 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_restart_ch6_st(
        &mut self,
    ) -> LedcTaskGammaRestartCh6StW<'_, TaskSt2Spec> {
        LedcTaskGammaRestartCh6StW::new(self, 10)
    }
    #[doc = "Bit 11 - Represents LEDC_task_gamma_restart_ch7 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_restart_ch7_st(
        &mut self,
    ) -> LedcTaskGammaRestartCh7StW<'_, TaskSt2Spec> {
        LedcTaskGammaRestartCh7StW::new(self, 11)
    }
    #[doc = "Bit 12 - Represents LEDC_task_gamma_pause_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_pause_ch0_st(&mut self) -> LedcTaskGammaPauseCh0StW<'_, TaskSt2Spec> {
        LedcTaskGammaPauseCh0StW::new(self, 12)
    }
    #[doc = "Bit 13 - Represents LEDC_task_gamma_pause_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_pause_ch1_st(&mut self) -> LedcTaskGammaPauseCh1StW<'_, TaskSt2Spec> {
        LedcTaskGammaPauseCh1StW::new(self, 13)
    }
    #[doc = "Bit 14 - Represents LEDC_task_gamma_pause_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_pause_ch2_st(&mut self) -> LedcTaskGammaPauseCh2StW<'_, TaskSt2Spec> {
        LedcTaskGammaPauseCh2StW::new(self, 14)
    }
    #[doc = "Bit 15 - Represents LEDC_task_gamma_pause_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_pause_ch3_st(&mut self) -> LedcTaskGammaPauseCh3StW<'_, TaskSt2Spec> {
        LedcTaskGammaPauseCh3StW::new(self, 15)
    }
    #[doc = "Bit 16 - Represents LEDC_task_gamma_pause_ch4 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_pause_ch4_st(&mut self) -> LedcTaskGammaPauseCh4StW<'_, TaskSt2Spec> {
        LedcTaskGammaPauseCh4StW::new(self, 16)
    }
    #[doc = "Bit 17 - Represents LEDC_task_gamma_pause_ch5 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_pause_ch5_st(&mut self) -> LedcTaskGammaPauseCh5StW<'_, TaskSt2Spec> {
        LedcTaskGammaPauseCh5StW::new(self, 17)
    }
    #[doc = "Bit 18 - Represents LEDC_task_gamma_pause_ch6 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_pause_ch6_st(&mut self) -> LedcTaskGammaPauseCh6StW<'_, TaskSt2Spec> {
        LedcTaskGammaPauseCh6StW::new(self, 18)
    }
    #[doc = "Bit 19 - Represents LEDC_task_gamma_pause_ch7 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_pause_ch7_st(&mut self) -> LedcTaskGammaPauseCh7StW<'_, TaskSt2Spec> {
        LedcTaskGammaPauseCh7StW::new(self, 19)
    }
    #[doc = "Bit 20 - Represents LEDC_task_gamma_resume_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_resume_ch0_st(&mut self) -> LedcTaskGammaResumeCh0StW<'_, TaskSt2Spec> {
        LedcTaskGammaResumeCh0StW::new(self, 20)
    }
    #[doc = "Bit 21 - Represents LEDC_task_gamma_resume_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_resume_ch1_st(&mut self) -> LedcTaskGammaResumeCh1StW<'_, TaskSt2Spec> {
        LedcTaskGammaResumeCh1StW::new(self, 21)
    }
    #[doc = "Bit 22 - Represents LEDC_task_gamma_resume_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_resume_ch2_st(&mut self) -> LedcTaskGammaResumeCh2StW<'_, TaskSt2Spec> {
        LedcTaskGammaResumeCh2StW::new(self, 22)
    }
    #[doc = "Bit 23 - Represents LEDC_task_gamma_resume_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_resume_ch3_st(&mut self) -> LedcTaskGammaResumeCh3StW<'_, TaskSt2Spec> {
        LedcTaskGammaResumeCh3StW::new(self, 23)
    }
    #[doc = "Bit 24 - Represents LEDC_task_gamma_resume_ch4 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_resume_ch4_st(&mut self) -> LedcTaskGammaResumeCh4StW<'_, TaskSt2Spec> {
        LedcTaskGammaResumeCh4StW::new(self, 24)
    }
    #[doc = "Bit 25 - Represents LEDC_task_gamma_resume_ch5 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_resume_ch5_st(&mut self) -> LedcTaskGammaResumeCh5StW<'_, TaskSt2Spec> {
        LedcTaskGammaResumeCh5StW::new(self, 25)
    }
    #[doc = "Bit 26 - Represents LEDC_task_gamma_resume_ch6 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_resume_ch6_st(&mut self) -> LedcTaskGammaResumeCh6StW<'_, TaskSt2Spec> {
        LedcTaskGammaResumeCh6StW::new(self, 26)
    }
    #[doc = "Bit 27 - Represents LEDC_task_gamma_resume_ch7 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ledc_task_gamma_resume_ch7_st(&mut self) -> LedcTaskGammaResumeCh7StW<'_, TaskSt2Spec> {
        LedcTaskGammaResumeCh7StW::new(self, 27)
    }
    #[doc = "Bit 28 - Represents TG0_task_cnt_start_timer0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn tg0_task_cnt_start_timer0_st(&mut self) -> Tg0TaskCntStartTimer0StW<'_, TaskSt2Spec> {
        Tg0TaskCntStartTimer0StW::new(self, 28)
    }
    #[doc = "Bit 29 - Represents TG0_task_alarm_start_timer0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn tg0_task_alarm_start_timer0_st(
        &mut self,
    ) -> Tg0TaskAlarmStartTimer0StW<'_, TaskSt2Spec> {
        Tg0TaskAlarmStartTimer0StW::new(self, 29)
    }
    #[doc = "Bit 30 - Represents TG0_task_cnt_stop_timer0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn tg0_task_cnt_stop_timer0_st(&mut self) -> Tg0TaskCntStopTimer0StW<'_, TaskSt2Spec> {
        Tg0TaskCntStopTimer0StW::new(self, 30)
    }
    #[doc = "Bit 31 - Represents TG0_task_cnt_reload_timer0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn tg0_task_cnt_reload_timer0_st(&mut self) -> Tg0TaskCntReloadTimer0StW<'_, TaskSt2Spec> {
        Tg0TaskCntReloadTimer0StW::new(self, 31)
    }
}
#[doc = "Tasks trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`task_st2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TaskSt2Spec;
impl crate::RegisterSpec for TaskSt2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`task_st2::R`](R) reader structure"]
impl crate::Readable for TaskSt2Spec {}
#[doc = "`write(|w| ..)` method takes [`task_st2::W`](W) writer structure"]
impl crate::Writable for TaskSt2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASK_ST2 to value 0"]
impl crate::Resettable for TaskSt2Spec {}
