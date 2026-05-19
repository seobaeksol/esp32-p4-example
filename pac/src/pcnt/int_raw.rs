#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<IntRawSpec>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<IntRawSpec>;
#[doc = "Field `CNT_THR_EVENT_U(0-3)` reader - The raw interrupt status bit for the PCNT_CNT_THR_EVENT_U%s_INT interrupt."]
pub type CntThrEventUR = crate::BitReader;
#[doc = "Field `CNT_THR_EVENT_U(0-3)` writer - The raw interrupt status bit for the PCNT_CNT_THR_EVENT_U%s_INT interrupt."]
pub type CntThrEventUW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "The raw interrupt status bit for the PCNT_CNT_THR_EVENT_U(0-3)_INT interrupt."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CNT_THR_EVENT_U0` field.</div>"]
    #[inline(always)]
    pub fn cnt_thr_event_u(&self, n: u8) -> CntThrEventUR {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CntThrEventUR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The raw interrupt status bit for the PCNT_CNT_THR_EVENT_U(0-3)_INT interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u_iter(&self) -> impl Iterator<Item = CntThrEventUR> + '_ {
        (0..4).map(move |n| CntThrEventUR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - The raw interrupt status bit for the PCNT_CNT_THR_EVENT_U0_INT interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u0(&self) -> CntThrEventUR {
        CntThrEventUR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for the PCNT_CNT_THR_EVENT_U1_INT interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u1(&self) -> CntThrEventUR {
        CntThrEventUR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status bit for the PCNT_CNT_THR_EVENT_U2_INT interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u2(&self) -> CntThrEventUR {
        CntThrEventUR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status bit for the PCNT_CNT_THR_EVENT_U3_INT interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u3(&self) -> CntThrEventUR {
        CntThrEventUR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "The raw interrupt status bit for the PCNT_CNT_THR_EVENT_U(0-3)_INT interrupt."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CNT_THR_EVENT_U0` field.</div>"]
    #[inline(always)]
    pub fn cnt_thr_event_u(&mut self, n: u8) -> CntThrEventUW<'_, IntRawSpec> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CntThrEventUW::new(self, n)
    }
    #[doc = "Bit 0 - The raw interrupt status bit for the PCNT_CNT_THR_EVENT_U0_INT interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u0(&mut self) -> CntThrEventUW<'_, IntRawSpec> {
        CntThrEventUW::new(self, 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for the PCNT_CNT_THR_EVENT_U1_INT interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u1(&mut self) -> CntThrEventUW<'_, IntRawSpec> {
        CntThrEventUW::new(self, 1)
    }
    #[doc = "Bit 2 - The raw interrupt status bit for the PCNT_CNT_THR_EVENT_U2_INT interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u2(&mut self) -> CntThrEventUW<'_, IntRawSpec> {
        CntThrEventUW::new(self, 2)
    }
    #[doc = "Bit 3 - The raw interrupt status bit for the PCNT_CNT_THR_EVENT_U3_INT interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u3(&mut self) -> CntThrEventUW<'_, IntRawSpec> {
        CntThrEventUW::new(self, 3)
    }
}
#[doc = "Interrupt raw status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntRawSpec;
impl crate::RegisterSpec for IntRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for IntRawSpec {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for IntRawSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for IntRawSpec {}
