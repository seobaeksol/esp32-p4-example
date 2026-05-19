#[doc = "Register `INT_ATTR%s` reader"]
pub type R = crate::R<IntAttrSpec>;
#[doc = "Register `INT_ATTR%s` writer"]
pub type W = crate::W<IntAttrSpec>;
#[doc = "Configures hardware vectoring for the ith CLIC machine mode interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shv {
    #[doc = "0: Not hardware vectored. Upon taking this interrupt, the CPU will jump to the address configured in mtvec."]
    Software = 0,
    #[doc = "1: Hardware vectored. Upon taking this interrupt, the CPU will jump to the word address stored at (mtvt + 4*i) relative to the base address configured in mtvt."]
    Hardware = 1,
}
impl From<Shv> for bool {
    #[inline(always)]
    fn from(variant: Shv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHV` reader - Configures hardware vectoring for the ith CLIC machine mode interrupt."]
pub type ShvR = crate::BitReader<Shv>;
impl ShvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Shv {
        match self.bits {
            false => Shv::Software,
            true => Shv::Hardware,
        }
    }
    #[doc = "Not hardware vectored. Upon taking this interrupt, the CPU will jump to the address configured in mtvec."]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == Shv::Software
    }
    #[doc = "Hardware vectored. Upon taking this interrupt, the CPU will jump to the word address stored at (mtvt + 4*i) relative to the base address configured in mtvt."]
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == Shv::Hardware
    }
}
#[doc = "Field `SHV` writer - Configures hardware vectoring for the ith CLIC machine mode interrupt."]
pub type ShvW<'a, REG> = crate::BitWriter<'a, REG, Shv>;
impl<'a, REG> ShvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not hardware vectored. Upon taking this interrupt, the CPU will jump to the address configured in mtvec."]
    #[inline(always)]
    pub fn software(self) -> &'a mut crate::W<REG> {
        self.variant(Shv::Software)
    }
    #[doc = "Hardware vectored. Upon taking this interrupt, the CPU will jump to the word address stored at (mtvt + 4*i) relative to the base address configured in mtvt."]
    #[inline(always)]
    pub fn hardware(self) -> &'a mut crate::W<REG> {
        self.variant(Shv::Hardware)
    }
}
#[doc = "Configures the trigger type and polarity of the ith CLIC machine mode interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trig {
    #[doc = "0: interrupt is positive level-triggered"]
    PositiveLevel = 0,
    #[doc = "1: interrupt is positive edge-triggered"]
    PositiveEdge = 1,
    #[doc = "2: interrupt is negative level-triggered"]
    NegativeLevel = 2,
    #[doc = "3: interrupt is negative edge-triggered"]
    NegativeEdge = 3,
}
impl From<Trig> for u8 {
    #[inline(always)]
    fn from(variant: Trig) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trig {
    type Ux = u8;
}
impl crate::IsEnum for Trig {}
#[doc = "Field `TRIG` reader - Configures the trigger type and polarity of the ith CLIC machine mode interrupt."]
pub type TrigR = crate::FieldReader<Trig>;
impl TrigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trig {
        match self.bits {
            0 => Trig::PositiveLevel,
            1 => Trig::PositiveEdge,
            2 => Trig::NegativeLevel,
            3 => Trig::NegativeEdge,
            _ => unreachable!(),
        }
    }
    #[doc = "interrupt is positive level-triggered"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == Trig::PositiveLevel
    }
    #[doc = "interrupt is positive edge-triggered"]
    #[inline(always)]
    pub fn is_positive_edge(&self) -> bool {
        *self == Trig::PositiveEdge
    }
    #[doc = "interrupt is negative level-triggered"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == Trig::NegativeLevel
    }
    #[doc = "interrupt is negative edge-triggered"]
    #[inline(always)]
    pub fn is_negative_edge(&self) -> bool {
        *self == Trig::NegativeEdge
    }
}
#[doc = "Field `TRIG` writer - Configures the trigger type and polarity of the ith CLIC machine mode interrupt."]
pub type TrigW<'a, REG> = crate::FieldWriter<'a, REG, 2, Trig, crate::Safe>;
impl<'a, REG> TrigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "interrupt is positive level-triggered"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut crate::W<REG> {
        self.variant(Trig::PositiveLevel)
    }
    #[doc = "interrupt is positive edge-triggered"]
    #[inline(always)]
    pub fn positive_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Trig::PositiveEdge)
    }
    #[doc = "interrupt is negative level-triggered"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut crate::W<REG> {
        self.variant(Trig::NegativeLevel)
    }
    #[doc = "interrupt is negative edge-triggered"]
    #[inline(always)]
    pub fn negative_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Trig::NegativeEdge)
    }
}
#[doc = "Field `MODE` reader - Configures the privilege mode of the ith CLIC interrupt. This is hardwired to 0x3 as user mode interrupts are not supported."]
pub type ModeR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Configures hardware vectoring for the ith CLIC machine mode interrupt."]
    #[inline(always)]
    pub fn shv(&self) -> ShvR {
        ShvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Configures the trigger type and polarity of the ith CLIC machine mode interrupt."]
    #[inline(always)]
    pub fn trig(&self) -> TrigR {
        TrigR::new((self.bits >> 1) & 3)
    }
    #[doc = "Bits 6:7 - Configures the privilege mode of the ith CLIC interrupt. This is hardwired to 0x3 as user mode interrupts are not supported."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Configures hardware vectoring for the ith CLIC machine mode interrupt."]
    #[inline(always)]
    pub fn shv(&mut self) -> ShvW<'_, IntAttrSpec> {
        ShvW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Configures the trigger type and polarity of the ith CLIC machine mode interrupt."]
    #[inline(always)]
    pub fn trig(&mut self) -> TrigW<'_, IntAttrSpec> {
        TrigW::new(self, 1)
    }
}
#[doc = "Interrupt attribute register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_attr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_attr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntAttrSpec;
impl crate::RegisterSpec for IntAttrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`int_attr::R`](R) reader structure"]
impl crate::Readable for IntAttrSpec {}
#[doc = "`write(|w| ..)` method takes [`int_attr::W`](W) writer structure"]
impl crate::Writable for IntAttrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ATTR%s to value 0"]
impl crate::Resettable for IntAttrSpec {}
