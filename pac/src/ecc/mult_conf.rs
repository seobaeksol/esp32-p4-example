#[doc = "Register `MULT_CONF` reader"]
pub type R = crate::R<MultConfSpec>;
#[doc = "Register `MULT_CONF` writer"]
pub type W = crate::W<MultConfSpec>;
#[doc = "Field `START` reader - Configures whether to start calculation of ECC Accelerator. This bit will be self-cleared after the calculation is done. \\\\ 0: No effect\\\\ 1: Start calculation of ECC Accelerator\\\\"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - Configures whether to start calculation of ECC Accelerator. This bit will be self-cleared after the calculation is done. \\\\ 0: No effect\\\\ 1: Start calculation of ECC Accelerator\\\\"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` writer - Configures whether to reset ECC Accelerator. \\\\ 0: No effect\\\\ 1: Reset\\\\"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Configures the key length mode bit of ECC Accelerator. \\\\ 0: P-192\\\\ 1: P-256\\\\ 2: P-384\\\\ 3: Reserved.\\\\\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KeyLength {
    #[doc = "0: P-192 elliptic curve"]
    P192 = 0,
    #[doc = "1: P-256 elliptic curve"]
    P256 = 1,
    #[doc = "2: P-384 elliptic curve"]
    P384 = 2,
}
impl From<KeyLength> for u8 {
    #[inline(always)]
    fn from(variant: KeyLength) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for KeyLength {
    type Ux = u8;
}
impl crate::IsEnum for KeyLength {}
#[doc = "Field `KEY_LENGTH` reader - Configures the key length mode bit of ECC Accelerator. \\\\ 0: P-192\\\\ 1: P-256\\\\ 2: P-384\\\\ 3: Reserved.\\\\"]
pub type KeyLengthR = crate::FieldReader<KeyLength>;
impl KeyLengthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<KeyLength> {
        match self.bits {
            0 => Some(KeyLength::P192),
            1 => Some(KeyLength::P256),
            2 => Some(KeyLength::P384),
            _ => None,
        }
    }
    #[doc = "P-192 elliptic curve"]
    #[inline(always)]
    pub fn is_p192(&self) -> bool {
        *self == KeyLength::P192
    }
    #[doc = "P-256 elliptic curve"]
    #[inline(always)]
    pub fn is_p256(&self) -> bool {
        *self == KeyLength::P256
    }
    #[doc = "P-384 elliptic curve"]
    #[inline(always)]
    pub fn is_p384(&self) -> bool {
        *self == KeyLength::P384
    }
}
#[doc = "Field `KEY_LENGTH` writer - Configures the key length mode bit of ECC Accelerator. \\\\ 0: P-192\\\\ 1: P-256\\\\ 2: P-384\\\\ 3: Reserved.\\\\"]
pub type KeyLengthW<'a, REG> = crate::FieldWriter<'a, REG, 2, KeyLength>;
impl<'a, REG> KeyLengthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "P-192 elliptic curve"]
    #[inline(always)]
    pub fn p192(self) -> &'a mut crate::W<REG> {
        self.variant(KeyLength::P192)
    }
    #[doc = "P-256 elliptic curve"]
    #[inline(always)]
    pub fn p256(self) -> &'a mut crate::W<REG> {
        self.variant(KeyLength::P256)
    }
    #[doc = "P-384 elliptic curve"]
    #[inline(always)]
    pub fn p384(self) -> &'a mut crate::W<REG> {
        self.variant(KeyLength::P384)
    }
}
#[doc = "Field `MOD_BASE` reader - Configures the mod base of mod operation, only valid in work_mode 8-11. \\\\ 0: n(order of curve)\\\\ 1: p(mod base of curve)\\\\"]
pub type ModBaseR = crate::BitReader;
#[doc = "Field `MOD_BASE` writer - Configures the mod base of mod operation, only valid in work_mode 8-11. \\\\ 0: n(order of curve)\\\\ 1: p(mod base of curve)\\\\"]
pub type ModBaseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WORK_MODE` reader - Configures the work mode of ECC Accelerator.\\\\ 0: Point Multi mode\\\\ 1: Reserved\\\\ 2: Point Verif mode\\\\ 3: Point Verif + Multi mode\\\\ 4: Jacobian Point Multi mode\\\\ 5: Reserved\\\\ 6: Jacobian Point Verif mode\\\\ 7: Point Verif + Jacobian Point Multi mode\\\\ 8: Mod Add mode\\\\ 9. Mod Sub mode\\\\ 10: Mod Multi mode\\\\ 11: Mod Div mode\\\\"]
pub type WorkModeR = crate::FieldReader;
#[doc = "Field `WORK_MODE` writer - Configures the work mode of ECC Accelerator.\\\\ 0: Point Multi mode\\\\ 1: Reserved\\\\ 2: Point Verif mode\\\\ 3: Point Verif + Multi mode\\\\ 4: Jacobian Point Multi mode\\\\ 5: Reserved\\\\ 6: Jacobian Point Verif mode\\\\ 7: Point Verif + Jacobian Point Multi mode\\\\ 8: Mod Add mode\\\\ 9. Mod Sub mode\\\\ 10: Mod Multi mode\\\\ 11: Mod Div mode\\\\"]
pub type WorkModeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SECURITY_MODE` reader - Configures the security mode of ECC Accelerator.\\\\ 0: no secure function enabled.\\\\ 1: enable constant-time calculation in all point multiplication modes.\\\\"]
pub type SecurityModeR = crate::BitReader;
#[doc = "Field `SECURITY_MODE` writer - Configures the security mode of ECC Accelerator.\\\\ 0: no secure function enabled.\\\\ 1: enable constant-time calculation in all point multiplication modes.\\\\"]
pub type SecurityModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VERIFICATION_RESULT` reader - Represents the verification result of ECC Accelerator, valid only when calculation is done."]
pub type VerificationResultR = crate::BitReader;
#[doc = "Field `CLK_EN` reader - Configures whether to force on register clock gate. \\\\ 0: No effect\\\\ 1: Force on\\\\"]
pub type ClkEnR = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Configures whether to force on register clock gate. \\\\ 0: No effect\\\\ 1: Force on\\\\"]
pub type ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_CLOCK_GATE_FORCE_ON` reader - Configures whether to force on ECC memory clock gate. \\\\ 0: No effect\\\\ 1: Force on\\\\"]
pub type MemClockGateForceOnR = crate::BitReader;
#[doc = "Field `MEM_CLOCK_GATE_FORCE_ON` writer - Configures whether to force on ECC memory clock gate. \\\\ 0: No effect\\\\ 1: Force on\\\\"]
pub type MemClockGateForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether to start calculation of ECC Accelerator. This bit will be self-cleared after the calculation is done. \\\\ 0: No effect\\\\ 1: Start calculation of ECC Accelerator\\\\"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Configures the key length mode bit of ECC Accelerator. \\\\ 0: P-192\\\\ 1: P-256\\\\ 2: P-384\\\\ 3: Reserved.\\\\"]
    #[inline(always)]
    pub fn key_length(&self) -> KeyLengthR {
        KeyLengthR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Configures the mod base of mod operation, only valid in work_mode 8-11. \\\\ 0: n(order of curve)\\\\ 1: p(mod base of curve)\\\\"]
    #[inline(always)]
    pub fn mod_base(&self) -> ModBaseR {
        ModBaseR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:8 - Configures the work mode of ECC Accelerator.\\\\ 0: Point Multi mode\\\\ 1: Reserved\\\\ 2: Point Verif mode\\\\ 3: Point Verif + Multi mode\\\\ 4: Jacobian Point Multi mode\\\\ 5: Reserved\\\\ 6: Jacobian Point Verif mode\\\\ 7: Point Verif + Jacobian Point Multi mode\\\\ 8: Mod Add mode\\\\ 9. Mod Sub mode\\\\ 10: Mod Multi mode\\\\ 11: Mod Div mode\\\\"]
    #[inline(always)]
    pub fn work_mode(&self) -> WorkModeR {
        WorkModeR::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Configures the security mode of ECC Accelerator.\\\\ 0: no secure function enabled.\\\\ 1: enable constant-time calculation in all point multiplication modes.\\\\"]
    #[inline(always)]
    pub fn security_mode(&self) -> SecurityModeR {
        SecurityModeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 29 - Represents the verification result of ECC Accelerator, valid only when calculation is done."]
    #[inline(always)]
    pub fn verification_result(&self) -> VerificationResultR {
        VerificationResultR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Configures whether to force on register clock gate. \\\\ 0: No effect\\\\ 1: Force on\\\\"]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Configures whether to force on ECC memory clock gate. \\\\ 0: No effect\\\\ 1: Force on\\\\"]
    #[inline(always)]
    pub fn mem_clock_gate_force_on(&self) -> MemClockGateForceOnR {
        MemClockGateForceOnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether to start calculation of ECC Accelerator. This bit will be self-cleared after the calculation is done. \\\\ 0: No effect\\\\ 1: Start calculation of ECC Accelerator\\\\"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, MultConfSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether to reset ECC Accelerator. \\\\ 0: No effect\\\\ 1: Reset\\\\"]
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<'_, MultConfSpec> {
        ResetW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Configures the key length mode bit of ECC Accelerator. \\\\ 0: P-192\\\\ 1: P-256\\\\ 2: P-384\\\\ 3: Reserved.\\\\"]
    #[inline(always)]
    pub fn key_length(&mut self) -> KeyLengthW<'_, MultConfSpec> {
        KeyLengthW::new(self, 2)
    }
    #[doc = "Bit 4 - Configures the mod base of mod operation, only valid in work_mode 8-11. \\\\ 0: n(order of curve)\\\\ 1: p(mod base of curve)\\\\"]
    #[inline(always)]
    pub fn mod_base(&mut self) -> ModBaseW<'_, MultConfSpec> {
        ModBaseW::new(self, 4)
    }
    #[doc = "Bits 5:8 - Configures the work mode of ECC Accelerator.\\\\ 0: Point Multi mode\\\\ 1: Reserved\\\\ 2: Point Verif mode\\\\ 3: Point Verif + Multi mode\\\\ 4: Jacobian Point Multi mode\\\\ 5: Reserved\\\\ 6: Jacobian Point Verif mode\\\\ 7: Point Verif + Jacobian Point Multi mode\\\\ 8: Mod Add mode\\\\ 9. Mod Sub mode\\\\ 10: Mod Multi mode\\\\ 11: Mod Div mode\\\\"]
    #[inline(always)]
    pub fn work_mode(&mut self) -> WorkModeW<'_, MultConfSpec> {
        WorkModeW::new(self, 5)
    }
    #[doc = "Bit 9 - Configures the security mode of ECC Accelerator.\\\\ 0: no secure function enabled.\\\\ 1: enable constant-time calculation in all point multiplication modes.\\\\"]
    #[inline(always)]
    pub fn security_mode(&mut self) -> SecurityModeW<'_, MultConfSpec> {
        SecurityModeW::new(self, 9)
    }
    #[doc = "Bit 30 - Configures whether to force on register clock gate. \\\\ 0: No effect\\\\ 1: Force on\\\\"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> ClkEnW<'_, MultConfSpec> {
        ClkEnW::new(self, 30)
    }
    #[doc = "Bit 31 - Configures whether to force on ECC memory clock gate. \\\\ 0: No effect\\\\ 1: Force on\\\\"]
    #[inline(always)]
    pub fn mem_clock_gate_force_on(&mut self) -> MemClockGateForceOnW<'_, MultConfSpec> {
        MemClockGateForceOnW::new(self, 31)
    }
}
#[doc = "ECC configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`mult_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mult_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MultConfSpec;
impl crate::RegisterSpec for MultConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mult_conf::R`](R) reader structure"]
impl crate::Readable for MultConfSpec {}
#[doc = "`write(|w| ..)` method takes [`mult_conf::W`](W) writer structure"]
impl crate::Writable for MultConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MULT_CONF to value 0"]
impl crate::Resettable for MultConfSpec {}
