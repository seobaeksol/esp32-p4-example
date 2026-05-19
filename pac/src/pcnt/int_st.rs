#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<IntStSpec>;
#[doc = "Field `CNT_THR_EVENT_U(0-3)` reader - The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U%s_INT interrupt."]
pub type CntThrEventUR = crate::BitReader;
impl R {
    #[doc = "The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U(0-3)_INT interrupt."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CNT_THR_EVENT_U0` field.</div>"]
    #[inline(always)]
    pub fn cnt_thr_event_u(&self, n: u8) -> CntThrEventUR {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CntThrEventUR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U(0-3)_INT interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u_iter(&self) -> impl Iterator<Item = CntThrEventUR> + '_ {
        (0..4).map(move |n| CntThrEventUR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U0_INT interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u0(&self) -> CntThrEventUR {
        CntThrEventUR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U1_INT interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u1(&self) -> CntThrEventUR {
        CntThrEventUR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U2_INT interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u2(&self) -> CntThrEventUR {
        CntThrEventUR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U3_INT interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u3(&self) -> CntThrEventUR {
        CntThrEventUR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStSpec;
impl crate::RegisterSpec for IntStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for IntStSpec {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for IntStSpec {}
