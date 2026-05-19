#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `CNT_RST_U(0-3)` reader - Set this bit to clear unit %s's counter."]
pub type CntRstUR = crate::BitReader;
#[doc = "Field `CNT_RST_U(0-3)` writer - Set this bit to clear unit %s's counter."]
pub type CntRstUW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNT_PAUSE_U(0-3)` reader - Set this bit to freeze unit %s's counter."]
pub type CntPauseUR = crate::BitReader;
#[doc = "Field `CNT_PAUSE_U(0-3)` writer - Set this bit to freeze unit %s's counter."]
pub type CntPauseUW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DALTA_CHANGE_EN_U(0-3)` reader - Configures this bit to enable unit %s's step comparator."]
pub type DaltaChangeEnUR = crate::BitReader;
#[doc = "Field `DALTA_CHANGE_EN_U(0-3)` writer - Configures this bit to enable unit %s's step comparator."]
pub type DaltaChangeEnUW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - The registers clock gate enable signal of PCNT module. 1: the registers can be read and written by application. 0: the registers can not be read or written by application"]
pub type ClkEnR = crate::BitReader;
#[doc = "Field `CLK_EN` writer - The registers clock gate enable signal of PCNT module. 1: the registers can be read and written by application. 0: the registers can not be read or written by application"]
pub type ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Set this bit to clear unit (0-3)'s counter."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CNT_RST_U0` field.</div>"]
    #[inline(always)]
    pub fn cnt_rst_u(&self, n: u8) -> CntRstUR {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CntRstUR::new(((self.bits >> (n * 2)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Set this bit to clear unit (0-3)'s counter."]
    #[inline(always)]
    pub fn cnt_rst_u_iter(&self) -> impl Iterator<Item = CntRstUR> + '_ {
        (0..4).map(move |n| CntRstUR::new(((self.bits >> (n * 2)) & 1) != 0))
    }
    #[doc = "Bit 0 - Set this bit to clear unit 0's counter."]
    #[inline(always)]
    pub fn cnt_rst_u0(&self) -> CntRstUR {
        CntRstUR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to clear unit 1's counter."]
    #[inline(always)]
    pub fn cnt_rst_u1(&self) -> CntRstUR {
        CntRstUR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to clear unit 2's counter."]
    #[inline(always)]
    pub fn cnt_rst_u2(&self) -> CntRstUR {
        CntRstUR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to clear unit 3's counter."]
    #[inline(always)]
    pub fn cnt_rst_u3(&self) -> CntRstUR {
        CntRstUR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Set this bit to freeze unit (0-3)'s counter."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CNT_PAUSE_U0` field.</div>"]
    #[inline(always)]
    pub fn cnt_pause_u(&self, n: u8) -> CntPauseUR {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CntPauseUR::new(((self.bits >> (n * 2 + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Set this bit to freeze unit (0-3)'s counter."]
    #[inline(always)]
    pub fn cnt_pause_u_iter(&self) -> impl Iterator<Item = CntPauseUR> + '_ {
        (0..4).map(move |n| CntPauseUR::new(((self.bits >> (n * 2 + 1)) & 1) != 0))
    }
    #[doc = "Bit 1 - Set this bit to freeze unit 0's counter."]
    #[inline(always)]
    pub fn cnt_pause_u0(&self) -> CntPauseUR {
        CntPauseUR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to freeze unit 1's counter."]
    #[inline(always)]
    pub fn cnt_pause_u1(&self) -> CntPauseUR {
        CntPauseUR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to freeze unit 2's counter."]
    #[inline(always)]
    pub fn cnt_pause_u2(&self) -> CntPauseUR {
        CntPauseUR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to freeze unit 3's counter."]
    #[inline(always)]
    pub fn cnt_pause_u3(&self) -> CntPauseUR {
        CntPauseUR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Configures this bit to enable unit (0-3)'s step comparator."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DALTA_CHANGE_EN_U0` field.</div>"]
    #[inline(always)]
    pub fn dalta_change_en_u(&self, n: u8) -> DaltaChangeEnUR {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        DaltaChangeEnUR::new(((self.bits >> (n + 8)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Configures this bit to enable unit (0-3)'s step comparator."]
    #[inline(always)]
    pub fn dalta_change_en_u_iter(&self) -> impl Iterator<Item = DaltaChangeEnUR> + '_ {
        (0..4).map(move |n| DaltaChangeEnUR::new(((self.bits >> (n + 8)) & 1) != 0))
    }
    #[doc = "Bit 8 - Configures this bit to enable unit 0's step comparator."]
    #[inline(always)]
    pub fn dalta_change_en_u0(&self) -> DaltaChangeEnUR {
        DaltaChangeEnUR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Configures this bit to enable unit 1's step comparator."]
    #[inline(always)]
    pub fn dalta_change_en_u1(&self) -> DaltaChangeEnUR {
        DaltaChangeEnUR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Configures this bit to enable unit 2's step comparator."]
    #[inline(always)]
    pub fn dalta_change_en_u2(&self) -> DaltaChangeEnUR {
        DaltaChangeEnUR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Configures this bit to enable unit 3's step comparator."]
    #[inline(always)]
    pub fn dalta_change_en_u3(&self) -> DaltaChangeEnUR {
        DaltaChangeEnUR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - The registers clock gate enable signal of PCNT module. 1: the registers can be read and written by application. 0: the registers can not be read or written by application"]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Set this bit to clear unit (0-3)'s counter."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CNT_RST_U0` field.</div>"]
    #[inline(always)]
    pub fn cnt_rst_u(&mut self, n: u8) -> CntRstUW<'_, CtrlSpec> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CntRstUW::new(self, n * 2)
    }
    #[doc = "Bit 0 - Set this bit to clear unit 0's counter."]
    #[inline(always)]
    pub fn cnt_rst_u0(&mut self) -> CntRstUW<'_, CtrlSpec> {
        CntRstUW::new(self, 0)
    }
    #[doc = "Bit 2 - Set this bit to clear unit 1's counter."]
    #[inline(always)]
    pub fn cnt_rst_u1(&mut self) -> CntRstUW<'_, CtrlSpec> {
        CntRstUW::new(self, 2)
    }
    #[doc = "Bit 4 - Set this bit to clear unit 2's counter."]
    #[inline(always)]
    pub fn cnt_rst_u2(&mut self) -> CntRstUW<'_, CtrlSpec> {
        CntRstUW::new(self, 4)
    }
    #[doc = "Bit 6 - Set this bit to clear unit 3's counter."]
    #[inline(always)]
    pub fn cnt_rst_u3(&mut self) -> CntRstUW<'_, CtrlSpec> {
        CntRstUW::new(self, 6)
    }
    #[doc = "Set this bit to freeze unit (0-3)'s counter."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CNT_PAUSE_U0` field.</div>"]
    #[inline(always)]
    pub fn cnt_pause_u(&mut self, n: u8) -> CntPauseUW<'_, CtrlSpec> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CntPauseUW::new(self, n * 2 + 1)
    }
    #[doc = "Bit 1 - Set this bit to freeze unit 0's counter."]
    #[inline(always)]
    pub fn cnt_pause_u0(&mut self) -> CntPauseUW<'_, CtrlSpec> {
        CntPauseUW::new(self, 1)
    }
    #[doc = "Bit 3 - Set this bit to freeze unit 1's counter."]
    #[inline(always)]
    pub fn cnt_pause_u1(&mut self) -> CntPauseUW<'_, CtrlSpec> {
        CntPauseUW::new(self, 3)
    }
    #[doc = "Bit 5 - Set this bit to freeze unit 2's counter."]
    #[inline(always)]
    pub fn cnt_pause_u2(&mut self) -> CntPauseUW<'_, CtrlSpec> {
        CntPauseUW::new(self, 5)
    }
    #[doc = "Bit 7 - Set this bit to freeze unit 3's counter."]
    #[inline(always)]
    pub fn cnt_pause_u3(&mut self) -> CntPauseUW<'_, CtrlSpec> {
        CntPauseUW::new(self, 7)
    }
    #[doc = "Configures this bit to enable unit (0-3)'s step comparator."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DALTA_CHANGE_EN_U0` field.</div>"]
    #[inline(always)]
    pub fn dalta_change_en_u(&mut self, n: u8) -> DaltaChangeEnUW<'_, CtrlSpec> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        DaltaChangeEnUW::new(self, n + 8)
    }
    #[doc = "Bit 8 - Configures this bit to enable unit 0's step comparator."]
    #[inline(always)]
    pub fn dalta_change_en_u0(&mut self) -> DaltaChangeEnUW<'_, CtrlSpec> {
        DaltaChangeEnUW::new(self, 8)
    }
    #[doc = "Bit 9 - Configures this bit to enable unit 1's step comparator."]
    #[inline(always)]
    pub fn dalta_change_en_u1(&mut self) -> DaltaChangeEnUW<'_, CtrlSpec> {
        DaltaChangeEnUW::new(self, 9)
    }
    #[doc = "Bit 10 - Configures this bit to enable unit 2's step comparator."]
    #[inline(always)]
    pub fn dalta_change_en_u2(&mut self) -> DaltaChangeEnUW<'_, CtrlSpec> {
        DaltaChangeEnUW::new(self, 10)
    }
    #[doc = "Bit 11 - Configures this bit to enable unit 3's step comparator."]
    #[inline(always)]
    pub fn dalta_change_en_u3(&mut self) -> DaltaChangeEnUW<'_, CtrlSpec> {
        DaltaChangeEnUW::new(self, 11)
    }
    #[doc = "Bit 16 - The registers clock gate enable signal of PCNT module. 1: the registers can be read and written by application. 0: the registers can not be read or written by application"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> ClkEnW<'_, CtrlSpec> {
        ClkEnW::new(self, 16)
    }
}
#[doc = "Control register for all counters\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0x01"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x01;
}
