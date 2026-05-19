#[doc = "Register `COMD%s` reader"]
pub type R = crate::R<ComdSpec>;
#[doc = "Register `COMD%s` writer"]
pub type W = crate::W<ComdSpec>;
#[doc = "Field `BYTE_NUM` reader - Number of bytes to be sent or received for command %s."]
pub type ByteNumR = crate::FieldReader;
#[doc = "Field `BYTE_NUM` writer - Number of bytes to be sent or received for command %s."]
pub type ByteNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ACK_CHECK_EN` reader - Acknowledge check enable for command %s."]
pub type AckCheckEnR = crate::BitReader;
#[doc = "Field `ACK_CHECK_EN` writer - Acknowledge check enable for command %s."]
pub type AckCheckEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK_EXP` reader - Acknowledge expected for command %s."]
pub type AckExpR = crate::BitReader;
#[doc = "Field `ACK_EXP` writer - Acknowledge expected for command %s."]
pub type AckExpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK_VALUE` reader - Acknowledge value for command %s."]
pub type AckValueR = crate::BitReader;
#[doc = "Field `ACK_VALUE` writer - Acknowledge value for command %s."]
pub type AckValueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Opcode part of command %s.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    #[doc = "1: WRITE opcode"]
    Write = 1,
    #[doc = "2: STOP opcode"]
    Stop = 2,
    #[doc = "3: READ opcode"]
    Read = 3,
    #[doc = "4: END opcode"]
    End = 4,
    #[doc = "6: RSTART opcode"]
    Rstart = 6,
}
impl From<Opcode> for u8 {
    #[inline(always)]
    fn from(variant: Opcode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Opcode {
    type Ux = u8;
}
impl crate::IsEnum for Opcode {}
#[doc = "Field `OPCODE` reader - Opcode part of command %s."]
pub type OpcodeR = crate::FieldReader<Opcode>;
impl OpcodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Opcode> {
        match self.bits {
            1 => Some(Opcode::Write),
            2 => Some(Opcode::Stop),
            3 => Some(Opcode::Read),
            4 => Some(Opcode::End),
            6 => Some(Opcode::Rstart),
            _ => None,
        }
    }
    #[doc = "WRITE opcode"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == Opcode::Write
    }
    #[doc = "STOP opcode"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Opcode::Stop
    }
    #[doc = "READ opcode"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == Opcode::Read
    }
    #[doc = "END opcode"]
    #[inline(always)]
    pub fn is_end(&self) -> bool {
        *self == Opcode::End
    }
    #[doc = "RSTART opcode"]
    #[inline(always)]
    pub fn is_rstart(&self) -> bool {
        *self == Opcode::Rstart
    }
}
#[doc = "Field `OPCODE` writer - Opcode part of command %s."]
pub type OpcodeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Opcode>;
impl<'a, REG> OpcodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "WRITE opcode"]
    #[inline(always)]
    pub fn write(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::Write)
    }
    #[doc = "STOP opcode"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::Stop)
    }
    #[doc = "READ opcode"]
    #[inline(always)]
    pub fn read(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::Read)
    }
    #[doc = "END opcode"]
    #[inline(always)]
    pub fn end(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::End)
    }
    #[doc = "RSTART opcode"]
    #[inline(always)]
    pub fn rstart(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::Rstart)
    }
}
#[doc = "Field `COMMAND_DONE` reader - Represents whether command 0 is done in I2C Master mode.\\\\ 0: Not done \\\\ 1: Done \\\\"]
pub type CommandDoneR = crate::BitReader;
#[doc = "Field `COMMAND_DONE` writer - Represents whether command 0 is done in I2C Master mode.\\\\ 0: Not done \\\\ 1: Done \\\\"]
pub type CommandDoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Number of bytes to be sent or received for command %s."]
    #[inline(always)]
    pub fn byte_num(&self) -> ByteNumR {
        ByteNumR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Acknowledge check enable for command %s."]
    #[inline(always)]
    pub fn ack_check_en(&self) -> AckCheckEnR {
        AckCheckEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Acknowledge expected for command %s."]
    #[inline(always)]
    pub fn ack_exp(&self) -> AckExpR {
        AckExpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Acknowledge value for command %s."]
    #[inline(always)]
    pub fn ack_value(&self) -> AckValueR {
        AckValueR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13 - Opcode part of command %s."]
    #[inline(always)]
    pub fn opcode(&self) -> OpcodeR {
        OpcodeR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 31 - Represents whether command 0 is done in I2C Master mode.\\\\ 0: Not done \\\\ 1: Done \\\\"]
    #[inline(always)]
    pub fn command_done(&self) -> CommandDoneR {
        CommandDoneR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Number of bytes to be sent or received for command %s."]
    #[inline(always)]
    pub fn byte_num(&mut self) -> ByteNumW<'_, ComdSpec> {
        ByteNumW::new(self, 0)
    }
    #[doc = "Bit 8 - Acknowledge check enable for command %s."]
    #[inline(always)]
    pub fn ack_check_en(&mut self) -> AckCheckEnW<'_, ComdSpec> {
        AckCheckEnW::new(self, 8)
    }
    #[doc = "Bit 9 - Acknowledge expected for command %s."]
    #[inline(always)]
    pub fn ack_exp(&mut self) -> AckExpW<'_, ComdSpec> {
        AckExpW::new(self, 9)
    }
    #[doc = "Bit 10 - Acknowledge value for command %s."]
    #[inline(always)]
    pub fn ack_value(&mut self) -> AckValueW<'_, ComdSpec> {
        AckValueW::new(self, 10)
    }
    #[doc = "Bits 11:13 - Opcode part of command %s."]
    #[inline(always)]
    pub fn opcode(&mut self) -> OpcodeW<'_, ComdSpec> {
        OpcodeW::new(self, 11)
    }
    #[doc = "Bit 31 - Represents whether command 0 is done in I2C Master mode.\\\\ 0: Not done \\\\ 1: Done \\\\"]
    #[inline(always)]
    pub fn command_done(&mut self) -> CommandDoneW<'_, ComdSpec> {
        CommandDoneW::new(self, 31)
    }
}
#[doc = "I2C command register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`comd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ComdSpec;
impl crate::RegisterSpec for ComdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comd::R`](R) reader structure"]
impl crate::Readable for ComdSpec {}
#[doc = "`write(|w| ..)` method takes [`comd::W`](W) writer structure"]
impl crate::Writable for ComdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMD%s to value 0"]
impl crate::Resettable for ComdSpec {}
