#[doc = "Register `EMACCONFIG` reader"]
pub type R = crate::R<EmacconfigSpec>;
#[doc = "Register `EMACCONFIG` writer"]
pub type W = crate::W<EmacconfigSpec>;
#[doc = "Field `PLTF` reader - These bits control the number of preamble bytes that are added to the beginning of every Transmit frame. The preamble reduction occurs only when the MAC is operating in the full-duplex mode.2'b00: 7 bytes of preamble. 2'b01: 5 bytes of preamble. 2'b10: 3 bytes of preamble."]
pub type PltfR = crate::FieldReader;
#[doc = "Field `PLTF` writer - These bits control the number of preamble bytes that are added to the beginning of every Transmit frame. The preamble reduction occurs only when the MAC is operating in the full-duplex mode.2'b00: 7 bytes of preamble. 2'b01: 5 bytes of preamble. 2'b10: 3 bytes of preamble."]
pub type PltfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RX` reader - When this bit is set the receiver state machine of the MAC is enabled for receiving frames from the MII. When this bit is reset the MAC receive state machine is disabled after the completion of the reception of the current frame and does not receive any further frames from the MII."]
pub type RxR = crate::BitReader;
#[doc = "Field `RX` writer - When this bit is set the receiver state machine of the MAC is enabled for receiving frames from the MII. When this bit is reset the MAC receive state machine is disabled after the completion of the reception of the current frame and does not receive any further frames from the MII."]
pub type RxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX` reader - When this bit is set the transmit state machine of the MAC is enabled for transmission on the MII. When this bit is reset the MAC transmit state machine is disabled after the completion of the transmission of the current frame and does not transmit any further frames."]
pub type TxR = crate::BitReader;
#[doc = "Field `TX` writer - When this bit is set the transmit state machine of the MAC is enabled for transmission on the MII. When this bit is reset the MAC transmit state machine is disabled after the completion of the transmission of the current frame and does not transmit any further frames."]
pub type TxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEFERRALCHECK` reader - Deferral Check."]
pub type DeferralcheckR = crate::BitReader;
#[doc = "Field `DEFERRALCHECK` writer - Deferral Check."]
pub type DeferralcheckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BACKOFFLIMIT` reader - The Back-Off limit determines the random integer number (r) of slot time delays (512 bit times for 10/100 Mbps) for which the MAC waits before rescheduling a transmission attempt during retries after a collision. This bit is applicable only in the half-duplex mode. 00: k= min (n 10). 01: k = min (n 8). 10: k = min (n 4). 11: k = min (n 1) n = retransmission attempt. The random integer r takes the value in the Range 0 ~ 2000."]
pub type BackofflimitR = crate::FieldReader;
#[doc = "Field `BACKOFFLIMIT` writer - The Back-Off limit determines the random integer number (r) of slot time delays (512 bit times for 10/100 Mbps) for which the MAC waits before rescheduling a transmission attempt during retries after a collision. This bit is applicable only in the half-duplex mode. 00: k= min (n 10). 01: k = min (n 8). 10: k = min (n 4). 11: k = min (n 1) n = retransmission attempt. The random integer r takes the value in the Range 0 ~ 2000."]
pub type BackofflimitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCRCSTRIP` reader - When this bit is set the MAC strips the Pad or FCS field on the incoming frames only if the value of the length field is less than 1 536 bytes. All received frames with length field greater than or equal to 1 536 bytes are passed to the application without stripping the Pad or FCS field. When this bit is reset the MAC passes all incoming frames without modifying them to the Host."]
pub type PadcrcstripR = crate::BitReader;
#[doc = "Field `PADCRCSTRIP` writer - When this bit is set the MAC strips the Pad or FCS field on the incoming frames only if the value of the length field is less than 1 536 bytes. All received frames with length field greater than or equal to 1 536 bytes are passed to the application without stripping the Pad or FCS field. When this bit is reset the MAC passes all incoming frames without modifying them to the Host."]
pub type PadcrcstripW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETRY` reader - When this bit is set the MAC attempts only one transmission. When a collision occurs on the MII interface the MAC ignores the current frame transmission and reports a Frame Abort with excessive collision error in the transmit frame status. When this bit is reset the MAC attempts retries based on the settings of the BL field (Bits \\[6:5\\]). This bit is applicable only in the half-duplex Mode."]
pub type RetryR = crate::BitReader;
#[doc = "Field `RETRY` writer - When this bit is set the MAC attempts only one transmission. When a collision occurs on the MII interface the MAC ignores the current frame transmission and reports a Frame Abort with excessive collision error in the transmit frame status. When this bit is reset the MAC attempts retries based on the settings of the BL field (Bits \\[6:5\\]). This bit is applicable only in the half-duplex Mode."]
pub type RetryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIPCOFFLOAD` reader - When this bit is set the MAC calculates the 16-bit one's complement of the one's complement sum of all received Ethernet frame payloads. It also checks whether the IPv4 Header checksum (assumed to be bytes 25/26 or 29/30 (VLAN-tagged) of the received Ethernet frame) is correct for the received frame and gives the status in the receive status word. The MAC also appends the 16-bit checksum calculated for the IP header datagram payload (bytes after the IPv4 header) and appends it to the Ethernet frame transferred to the application (when Type 2 COE is deselected). When this bit is reset this function is disabled."]
pub type RxipcoffloadR = crate::BitReader;
#[doc = "Field `RXIPCOFFLOAD` writer - When this bit is set the MAC calculates the 16-bit one's complement of the one's complement sum of all received Ethernet frame payloads. It also checks whether the IPv4 Header checksum (assumed to be bytes 25/26 or 29/30 (VLAN-tagged) of the received Ethernet frame) is correct for the received frame and gives the status in the receive status word. The MAC also appends the 16-bit checksum calculated for the IP header datagram payload (bytes after the IPv4 header) and appends it to the Ethernet frame transferred to the application (when Type 2 COE is deselected). When this bit is reset this function is disabled."]
pub type RxipcoffloadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUPLEX` reader - When this bit is set the MAC operates in the full-duplex mode where it can transmit and receive simultaneously. This bit is read only with default value of 1'b1 in the full-duplex-mode."]
pub type DuplexR = crate::BitReader;
#[doc = "Field `DUPLEX` writer - When this bit is set the MAC operates in the full-duplex mode where it can transmit and receive simultaneously. This bit is read only with default value of 1'b1 in the full-duplex-mode."]
pub type DuplexW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOOPBACK` reader - When this bit is set the MAC operates in the loopback mode MII. The MII Receive clock input (CLK_RX) is required for the loopback to work properly because the transmit clock is not looped-back internally."]
pub type LoopbackR = crate::BitReader;
#[doc = "Field `LOOPBACK` writer - When this bit is set the MAC operates in the loopback mode MII. The MII Receive clock input (CLK_RX) is required for the loopback to work properly because the transmit clock is not looped-back internally."]
pub type LoopbackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOWN` reader - When this bit is set the MAC disables the reception of frames when the TX_EN is asserted in the half-duplex mode. When this bit is reset the MAC receives all packets that are given by the PHY while transmitting. This bit is not applicable if the MAC is operating in the full duplex mode."]
pub type RxownR = crate::BitReader;
#[doc = "Field `RXOWN` writer - When this bit is set the MAC disables the reception of frames when the TX_EN is asserted in the half-duplex mode. When this bit is reset the MAC receives all packets that are given by the PHY while transmitting. This bit is not applicable if the MAC is operating in the full duplex mode."]
pub type RxownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FESPEED` reader - This bit selects the speed in the MII RMII interface. 0: 10 Mbps. 1: 100 Mbps."]
pub type FespeedR = crate::BitReader;
#[doc = "Field `FESPEED` writer - This bit selects the speed in the MII RMII interface. 0: 10 Mbps. 1: 100 Mbps."]
pub type FespeedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MII` reader - This bit selects the Ethernet line speed. It should be set to 1 for 10 or 100 Mbps operations.In 10 or 100 Mbps operations this bit along with FES(EMACFESPEED) bit it selects the exact linespeed. In the 10/100 Mbps-only operations the bit is always 1."]
pub type MiiR = crate::BitReader;
#[doc = "Field `MII` writer - This bit selects the Ethernet line speed. It should be set to 1 for 10 or 100 Mbps operations.In 10 or 100 Mbps operations this bit along with FES(EMACFESPEED) bit it selects the exact linespeed. In the 10/100 Mbps-only operations the bit is always 1."]
pub type MiiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLECRS` reader - When set high this bit makes the MAC transmitter ignore the MII CRS signal during frame transmission in the half-duplex mode. This request results in no errors generated because of Loss of Carrier or No Carrier during such transmission. When this bit is low the MAC transmitter generates such errors because of Carrier Sense and can even abort the transmissions."]
pub type DisablecrsR = crate::BitReader;
#[doc = "Field `DISABLECRS` writer - When set high this bit makes the MAC transmitter ignore the MII CRS signal during frame transmission in the half-duplex mode. This request results in no errors generated because of Loss of Carrier or No Carrier during such transmission. When this bit is low the MAC transmitter generates such errors because of Carrier Sense and can even abort the transmissions."]
pub type DisablecrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERFRAMEGAP` reader - These bits control the minimum IFG between frames during transmission. 3'b000: 96 bit times. 3'b001: 88 bit times. 3'b010: 80 bit times. 3'b111: 40 bit times. In the half-duplex mode the minimum IFG can be configured only for 64 bit times (IFG = 100). Lower values are not considered."]
pub type InterframegapR = crate::FieldReader;
#[doc = "Field `INTERFRAMEGAP` writer - These bits control the minimum IFG between frames during transmission. 3'b000: 96 bit times. 3'b001: 88 bit times. 3'b010: 80 bit times. 3'b111: 40 bit times. In the half-duplex mode the minimum IFG can be configured only for 64 bit times (IFG = 100). Lower values are not considered."]
pub type InterframegapW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `JUMBOFRAME` reader - When this bit is set the MAC allows Jumbo frames of 9 018 bytes (9 022 bytes for VLAN tagged frames) without reporting a giant frame error in the receive frame status."]
pub type JumboframeR = crate::BitReader;
#[doc = "Field `JUMBOFRAME` writer - When this bit is set the MAC allows Jumbo frames of 9 018 bytes (9 022 bytes for VLAN tagged frames) without reporting a giant frame error in the receive frame status."]
pub type JumboframeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JABBER` reader - When this bit is set the MAC disables the jabber timer on the transmitter. The MAC can transfer frames of up to 16 383 bytes. When this bit is reset the MAC cuts off the transmitter if the application sends out more than 2 048 bytes of data (10 240 if JE is set high) during Transmission."]
pub type JabberR = crate::BitReader;
#[doc = "Field `JABBER` writer - When this bit is set the MAC disables the jabber timer on the transmitter. The MAC can transfer frames of up to 16 383 bytes. When this bit is reset the MAC cuts off the transmitter if the application sends out more than 2 048 bytes of data (10 240 if JE is set high) during Transmission."]
pub type JabberW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WATCHDOG` reader - When this bit is set the MAC disables the watchdog timer on the receiver. The MAC can receive frames of up to 16 383 bytes. When this bit is reset the MAC does not allow a receive frame which more than 2 048 bytes (10 240 if JE is set high) or the value programmed in Register (Watchdog Timeout Register). The MAC cuts off any bytes received after the watchdog limit number of bytes."]
pub type WatchdogR = crate::BitReader;
#[doc = "Field `WATCHDOG` writer - When this bit is set the MAC disables the watchdog timer on the receiver. The MAC can receive frames of up to 16 383 bytes. When this bit is reset the MAC does not allow a receive frame which more than 2 048 bytes (10 240 if JE is set high) or the value programmed in Register (Watchdog Timeout Register). The MAC cuts off any bytes received after the watchdog limit number of bytes."]
pub type WatchdogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASS2KP` reader - When set the MAC considers all frames with up to 2 000 bytes length as normal packets.When Bit\\[20\\] (JE) is not set the MAC considers all received frames of size more than 2K bytes as Giant frames. When this bit is reset and Bit\\[20\\] (JE) is not set the MAC considers all received frames of size more than 1 518 bytes (1 522 bytes for tagged) as Giant frames. When Bit\\[20\\] is set setting this bit has no effect on Giant Frame status."]
pub type Ass2kpR = crate::BitReader;
#[doc = "Field `ASS2KP` writer - When set the MAC considers all frames with up to 2 000 bytes length as normal packets.When Bit\\[20\\] (JE) is not set the MAC considers all received frames of size more than 2K bytes as Giant frames. When this bit is reset and Bit\\[20\\] (JE) is not set the MAC considers all received frames of size more than 1 518 bytes (1 522 bytes for tagged) as Giant frames. When Bit\\[20\\] is set setting this bit has no effect on Giant Frame status."]
pub type Ass2kpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAIRC` reader - This field controls the source address insertion or replacement for all transmitted frames.Bit\\[30\\] specifies which MAC Address register (0 or 1) is used for source address insertion or replacement based on the values of Bits \\[29:28\\]: 2'b0x: The input signals mti_sa_ctrl_i and ati_sa_ctrl_i control the SA field generation. 2'b10: If Bit\\[30\\] is set to 0 the MAC inserts the content of the MAC Address 0 registers in the SA field of all transmitted frames. If Bit\\[30\\] is set to 1 the MAC inserts the content of the MAC Address 1 registers in the SA field of all transmitted frames. 2'b11: If Bit\\[30\\] is set to 0 the MAC replaces the content of the MAC Address 0 registers in the SA field of all transmitted frames. If Bit\\[30\\] is set to 1 the MAC replaces the content of the MAC Address 1 registers in the SA field of all transmitted frames."]
pub type SaircR = crate::FieldReader;
#[doc = "Field `SAIRC` writer - This field controls the source address insertion or replacement for all transmitted frames.Bit\\[30\\] specifies which MAC Address register (0 or 1) is used for source address insertion or replacement based on the values of Bits \\[29:28\\]: 2'b0x: The input signals mti_sa_ctrl_i and ati_sa_ctrl_i control the SA field generation. 2'b10: If Bit\\[30\\] is set to 0 the MAC inserts the content of the MAC Address 0 registers in the SA field of all transmitted frames. If Bit\\[30\\] is set to 1 the MAC inserts the content of the MAC Address 1 registers in the SA field of all transmitted frames. 2'b11: If Bit\\[30\\] is set to 0 the MAC replaces the content of the MAC Address 0 registers in the SA field of all transmitted frames. If Bit\\[30\\] is set to 1 the MAC replaces the content of the MAC Address 1 registers in the SA field of all transmitted frames."]
pub type SaircW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - These bits control the number of preamble bytes that are added to the beginning of every Transmit frame. The preamble reduction occurs only when the MAC is operating in the full-duplex mode.2'b00: 7 bytes of preamble. 2'b01: 5 bytes of preamble. 2'b10: 3 bytes of preamble."]
    #[inline(always)]
    pub fn pltf(&self) -> PltfR {
        PltfR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - When this bit is set the receiver state machine of the MAC is enabled for receiving frames from the MII. When this bit is reset the MAC receive state machine is disabled after the completion of the reception of the current frame and does not receive any further frames from the MII."]
    #[inline(always)]
    pub fn rx(&self) -> RxR {
        RxR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When this bit is set the transmit state machine of the MAC is enabled for transmission on the MII. When this bit is reset the MAC transmit state machine is disabled after the completion of the transmission of the current frame and does not transmit any further frames."]
    #[inline(always)]
    pub fn tx(&self) -> TxR {
        TxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Deferral Check."]
    #[inline(always)]
    pub fn deferralcheck(&self) -> DeferralcheckR {
        DeferralcheckR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - The Back-Off limit determines the random integer number (r) of slot time delays (512 bit times for 10/100 Mbps) for which the MAC waits before rescheduling a transmission attempt during retries after a collision. This bit is applicable only in the half-duplex mode. 00: k= min (n 10). 01: k = min (n 8). 10: k = min (n 4). 11: k = min (n 1) n = retransmission attempt. The random integer r takes the value in the Range 0 ~ 2000."]
    #[inline(always)]
    pub fn backofflimit(&self) -> BackofflimitR {
        BackofflimitR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - When this bit is set the MAC strips the Pad or FCS field on the incoming frames only if the value of the length field is less than 1 536 bytes. All received frames with length field greater than or equal to 1 536 bytes are passed to the application without stripping the Pad or FCS field. When this bit is reset the MAC passes all incoming frames without modifying them to the Host."]
    #[inline(always)]
    pub fn padcrcstrip(&self) -> PadcrcstripR {
        PadcrcstripR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - When this bit is set the MAC attempts only one transmission. When a collision occurs on the MII interface the MAC ignores the current frame transmission and reports a Frame Abort with excessive collision error in the transmit frame status. When this bit is reset the MAC attempts retries based on the settings of the BL field (Bits \\[6:5\\]). This bit is applicable only in the half-duplex Mode."]
    #[inline(always)]
    pub fn retry(&self) -> RetryR {
        RetryR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When this bit is set the MAC calculates the 16-bit one's complement of the one's complement sum of all received Ethernet frame payloads. It also checks whether the IPv4 Header checksum (assumed to be bytes 25/26 or 29/30 (VLAN-tagged) of the received Ethernet frame) is correct for the received frame and gives the status in the receive status word. The MAC also appends the 16-bit checksum calculated for the IP header datagram payload (bytes after the IPv4 header) and appends it to the Ethernet frame transferred to the application (when Type 2 COE is deselected). When this bit is reset this function is disabled."]
    #[inline(always)]
    pub fn rxipcoffload(&self) -> RxipcoffloadR {
        RxipcoffloadR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - When this bit is set the MAC operates in the full-duplex mode where it can transmit and receive simultaneously. This bit is read only with default value of 1'b1 in the full-duplex-mode."]
    #[inline(always)]
    pub fn duplex(&self) -> DuplexR {
        DuplexR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - When this bit is set the MAC operates in the loopback mode MII. The MII Receive clock input (CLK_RX) is required for the loopback to work properly because the transmit clock is not looped-back internally."]
    #[inline(always)]
    pub fn loopback(&self) -> LoopbackR {
        LoopbackR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - When this bit is set the MAC disables the reception of frames when the TX_EN is asserted in the half-duplex mode. When this bit is reset the MAC receives all packets that are given by the PHY while transmitting. This bit is not applicable if the MAC is operating in the full duplex mode."]
    #[inline(always)]
    pub fn rxown(&self) -> RxownR {
        RxownR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This bit selects the speed in the MII RMII interface. 0: 10 Mbps. 1: 100 Mbps."]
    #[inline(always)]
    pub fn fespeed(&self) -> FespeedR {
        FespeedR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This bit selects the Ethernet line speed. It should be set to 1 for 10 or 100 Mbps operations.In 10 or 100 Mbps operations this bit along with FES(EMACFESPEED) bit it selects the exact linespeed. In the 10/100 Mbps-only operations the bit is always 1."]
    #[inline(always)]
    pub fn mii(&self) -> MiiR {
        MiiR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - When set high this bit makes the MAC transmitter ignore the MII CRS signal during frame transmission in the half-duplex mode. This request results in no errors generated because of Loss of Carrier or No Carrier during such transmission. When this bit is low the MAC transmitter generates such errors because of Carrier Sense and can even abort the transmissions."]
    #[inline(always)]
    pub fn disablecrs(&self) -> DisablecrsR {
        DisablecrsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - These bits control the minimum IFG between frames during transmission. 3'b000: 96 bit times. 3'b001: 88 bit times. 3'b010: 80 bit times. 3'b111: 40 bit times. In the half-duplex mode the minimum IFG can be configured only for 64 bit times (IFG = 100). Lower values are not considered."]
    #[inline(always)]
    pub fn interframegap(&self) -> InterframegapR {
        InterframegapR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - When this bit is set the MAC allows Jumbo frames of 9 018 bytes (9 022 bytes for VLAN tagged frames) without reporting a giant frame error in the receive frame status."]
    #[inline(always)]
    pub fn jumboframe(&self) -> JumboframeR {
        JumboframeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - When this bit is set the MAC disables the jabber timer on the transmitter. The MAC can transfer frames of up to 16 383 bytes. When this bit is reset the MAC cuts off the transmitter if the application sends out more than 2 048 bytes of data (10 240 if JE is set high) during Transmission."]
    #[inline(always)]
    pub fn jabber(&self) -> JabberR {
        JabberR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - When this bit is set the MAC disables the watchdog timer on the receiver. The MAC can receive frames of up to 16 383 bytes. When this bit is reset the MAC does not allow a receive frame which more than 2 048 bytes (10 240 if JE is set high) or the value programmed in Register (Watchdog Timeout Register). The MAC cuts off any bytes received after the watchdog limit number of bytes."]
    #[inline(always)]
    pub fn watchdog(&self) -> WatchdogR {
        WatchdogR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 27 - When set the MAC considers all frames with up to 2 000 bytes length as normal packets.When Bit\\[20\\] (JE) is not set the MAC considers all received frames of size more than 2K bytes as Giant frames. When this bit is reset and Bit\\[20\\] (JE) is not set the MAC considers all received frames of size more than 1 518 bytes (1 522 bytes for tagged) as Giant frames. When Bit\\[20\\] is set setting this bit has no effect on Giant Frame status."]
    #[inline(always)]
    pub fn ass2kp(&self) -> Ass2kpR {
        Ass2kpR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - This field controls the source address insertion or replacement for all transmitted frames.Bit\\[30\\] specifies which MAC Address register (0 or 1) is used for source address insertion or replacement based on the values of Bits \\[29:28\\]: 2'b0x: The input signals mti_sa_ctrl_i and ati_sa_ctrl_i control the SA field generation. 2'b10: If Bit\\[30\\] is set to 0 the MAC inserts the content of the MAC Address 0 registers in the SA field of all transmitted frames. If Bit\\[30\\] is set to 1 the MAC inserts the content of the MAC Address 1 registers in the SA field of all transmitted frames. 2'b11: If Bit\\[30\\] is set to 0 the MAC replaces the content of the MAC Address 0 registers in the SA field of all transmitted frames. If Bit\\[30\\] is set to 1 the MAC replaces the content of the MAC Address 1 registers in the SA field of all transmitted frames."]
    #[inline(always)]
    pub fn sairc(&self) -> SaircR {
        SaircR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - These bits control the number of preamble bytes that are added to the beginning of every Transmit frame. The preamble reduction occurs only when the MAC is operating in the full-duplex mode.2'b00: 7 bytes of preamble. 2'b01: 5 bytes of preamble. 2'b10: 3 bytes of preamble."]
    #[inline(always)]
    pub fn pltf(&mut self) -> PltfW<'_, EmacconfigSpec> {
        PltfW::new(self, 0)
    }
    #[doc = "Bit 2 - When this bit is set the receiver state machine of the MAC is enabled for receiving frames from the MII. When this bit is reset the MAC receive state machine is disabled after the completion of the reception of the current frame and does not receive any further frames from the MII."]
    #[inline(always)]
    pub fn rx(&mut self) -> RxW<'_, EmacconfigSpec> {
        RxW::new(self, 2)
    }
    #[doc = "Bit 3 - When this bit is set the transmit state machine of the MAC is enabled for transmission on the MII. When this bit is reset the MAC transmit state machine is disabled after the completion of the transmission of the current frame and does not transmit any further frames."]
    #[inline(always)]
    pub fn tx(&mut self) -> TxW<'_, EmacconfigSpec> {
        TxW::new(self, 3)
    }
    #[doc = "Bit 4 - Deferral Check."]
    #[inline(always)]
    pub fn deferralcheck(&mut self) -> DeferralcheckW<'_, EmacconfigSpec> {
        DeferralcheckW::new(self, 4)
    }
    #[doc = "Bits 5:6 - The Back-Off limit determines the random integer number (r) of slot time delays (512 bit times for 10/100 Mbps) for which the MAC waits before rescheduling a transmission attempt during retries after a collision. This bit is applicable only in the half-duplex mode. 00: k= min (n 10). 01: k = min (n 8). 10: k = min (n 4). 11: k = min (n 1) n = retransmission attempt. The random integer r takes the value in the Range 0 ~ 2000."]
    #[inline(always)]
    pub fn backofflimit(&mut self) -> BackofflimitW<'_, EmacconfigSpec> {
        BackofflimitW::new(self, 5)
    }
    #[doc = "Bit 7 - When this bit is set the MAC strips the Pad or FCS field on the incoming frames only if the value of the length field is less than 1 536 bytes. All received frames with length field greater than or equal to 1 536 bytes are passed to the application without stripping the Pad or FCS field. When this bit is reset the MAC passes all incoming frames without modifying them to the Host."]
    #[inline(always)]
    pub fn padcrcstrip(&mut self) -> PadcrcstripW<'_, EmacconfigSpec> {
        PadcrcstripW::new(self, 7)
    }
    #[doc = "Bit 9 - When this bit is set the MAC attempts only one transmission. When a collision occurs on the MII interface the MAC ignores the current frame transmission and reports a Frame Abort with excessive collision error in the transmit frame status. When this bit is reset the MAC attempts retries based on the settings of the BL field (Bits \\[6:5\\]). This bit is applicable only in the half-duplex Mode."]
    #[inline(always)]
    pub fn retry(&mut self) -> RetryW<'_, EmacconfigSpec> {
        RetryW::new(self, 9)
    }
    #[doc = "Bit 10 - When this bit is set the MAC calculates the 16-bit one's complement of the one's complement sum of all received Ethernet frame payloads. It also checks whether the IPv4 Header checksum (assumed to be bytes 25/26 or 29/30 (VLAN-tagged) of the received Ethernet frame) is correct for the received frame and gives the status in the receive status word. The MAC also appends the 16-bit checksum calculated for the IP header datagram payload (bytes after the IPv4 header) and appends it to the Ethernet frame transferred to the application (when Type 2 COE is deselected). When this bit is reset this function is disabled."]
    #[inline(always)]
    pub fn rxipcoffload(&mut self) -> RxipcoffloadW<'_, EmacconfigSpec> {
        RxipcoffloadW::new(self, 10)
    }
    #[doc = "Bit 11 - When this bit is set the MAC operates in the full-duplex mode where it can transmit and receive simultaneously. This bit is read only with default value of 1'b1 in the full-duplex-mode."]
    #[inline(always)]
    pub fn duplex(&mut self) -> DuplexW<'_, EmacconfigSpec> {
        DuplexW::new(self, 11)
    }
    #[doc = "Bit 12 - When this bit is set the MAC operates in the loopback mode MII. The MII Receive clock input (CLK_RX) is required for the loopback to work properly because the transmit clock is not looped-back internally."]
    #[inline(always)]
    pub fn loopback(&mut self) -> LoopbackW<'_, EmacconfigSpec> {
        LoopbackW::new(self, 12)
    }
    #[doc = "Bit 13 - When this bit is set the MAC disables the reception of frames when the TX_EN is asserted in the half-duplex mode. When this bit is reset the MAC receives all packets that are given by the PHY while transmitting. This bit is not applicable if the MAC is operating in the full duplex mode."]
    #[inline(always)]
    pub fn rxown(&mut self) -> RxownW<'_, EmacconfigSpec> {
        RxownW::new(self, 13)
    }
    #[doc = "Bit 14 - This bit selects the speed in the MII RMII interface. 0: 10 Mbps. 1: 100 Mbps."]
    #[inline(always)]
    pub fn fespeed(&mut self) -> FespeedW<'_, EmacconfigSpec> {
        FespeedW::new(self, 14)
    }
    #[doc = "Bit 15 - This bit selects the Ethernet line speed. It should be set to 1 for 10 or 100 Mbps operations.In 10 or 100 Mbps operations this bit along with FES(EMACFESPEED) bit it selects the exact linespeed. In the 10/100 Mbps-only operations the bit is always 1."]
    #[inline(always)]
    pub fn mii(&mut self) -> MiiW<'_, EmacconfigSpec> {
        MiiW::new(self, 15)
    }
    #[doc = "Bit 16 - When set high this bit makes the MAC transmitter ignore the MII CRS signal during frame transmission in the half-duplex mode. This request results in no errors generated because of Loss of Carrier or No Carrier during such transmission. When this bit is low the MAC transmitter generates such errors because of Carrier Sense and can even abort the transmissions."]
    #[inline(always)]
    pub fn disablecrs(&mut self) -> DisablecrsW<'_, EmacconfigSpec> {
        DisablecrsW::new(self, 16)
    }
    #[doc = "Bits 17:19 - These bits control the minimum IFG between frames during transmission. 3'b000: 96 bit times. 3'b001: 88 bit times. 3'b010: 80 bit times. 3'b111: 40 bit times. In the half-duplex mode the minimum IFG can be configured only for 64 bit times (IFG = 100). Lower values are not considered."]
    #[inline(always)]
    pub fn interframegap(&mut self) -> InterframegapW<'_, EmacconfigSpec> {
        InterframegapW::new(self, 17)
    }
    #[doc = "Bit 20 - When this bit is set the MAC allows Jumbo frames of 9 018 bytes (9 022 bytes for VLAN tagged frames) without reporting a giant frame error in the receive frame status."]
    #[inline(always)]
    pub fn jumboframe(&mut self) -> JumboframeW<'_, EmacconfigSpec> {
        JumboframeW::new(self, 20)
    }
    #[doc = "Bit 22 - When this bit is set the MAC disables the jabber timer on the transmitter. The MAC can transfer frames of up to 16 383 bytes. When this bit is reset the MAC cuts off the transmitter if the application sends out more than 2 048 bytes of data (10 240 if JE is set high) during Transmission."]
    #[inline(always)]
    pub fn jabber(&mut self) -> JabberW<'_, EmacconfigSpec> {
        JabberW::new(self, 22)
    }
    #[doc = "Bit 23 - When this bit is set the MAC disables the watchdog timer on the receiver. The MAC can receive frames of up to 16 383 bytes. When this bit is reset the MAC does not allow a receive frame which more than 2 048 bytes (10 240 if JE is set high) or the value programmed in Register (Watchdog Timeout Register). The MAC cuts off any bytes received after the watchdog limit number of bytes."]
    #[inline(always)]
    pub fn watchdog(&mut self) -> WatchdogW<'_, EmacconfigSpec> {
        WatchdogW::new(self, 23)
    }
    #[doc = "Bit 27 - When set the MAC considers all frames with up to 2 000 bytes length as normal packets.When Bit\\[20\\] (JE) is not set the MAC considers all received frames of size more than 2K bytes as Giant frames. When this bit is reset and Bit\\[20\\] (JE) is not set the MAC considers all received frames of size more than 1 518 bytes (1 522 bytes for tagged) as Giant frames. When Bit\\[20\\] is set setting this bit has no effect on Giant Frame status."]
    #[inline(always)]
    pub fn ass2kp(&mut self) -> Ass2kpW<'_, EmacconfigSpec> {
        Ass2kpW::new(self, 27)
    }
    #[doc = "Bits 28:30 - This field controls the source address insertion or replacement for all transmitted frames.Bit\\[30\\] specifies which MAC Address register (0 or 1) is used for source address insertion or replacement based on the values of Bits \\[29:28\\]: 2'b0x: The input signals mti_sa_ctrl_i and ati_sa_ctrl_i control the SA field generation. 2'b10: If Bit\\[30\\] is set to 0 the MAC inserts the content of the MAC Address 0 registers in the SA field of all transmitted frames. If Bit\\[30\\] is set to 1 the MAC inserts the content of the MAC Address 1 registers in the SA field of all transmitted frames. 2'b11: If Bit\\[30\\] is set to 0 the MAC replaces the content of the MAC Address 0 registers in the SA field of all transmitted frames. If Bit\\[30\\] is set to 1 the MAC replaces the content of the MAC Address 1 registers in the SA field of all transmitted frames."]
    #[inline(always)]
    pub fn sairc(&mut self) -> SaircW<'_, EmacconfigSpec> {
        SaircW::new(self, 28)
    }
}
#[doc = "MAC configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`emacconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emacconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmacconfigSpec;
impl crate::RegisterSpec for EmacconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emacconfig::R`](R) reader structure"]
impl crate::Readable for EmacconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`emacconfig::W`](W) writer structure"]
impl crate::Writable for EmacconfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EMACCONFIG to value 0"]
impl crate::Resettable for EmacconfigSpec {}
