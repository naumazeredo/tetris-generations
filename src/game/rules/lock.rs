use crate::linalg::Vec2i;
use crate::game::{
    pieces::Piece,
    playfield::Playfield,
};

use super::*;

#[derive(Copy, Clone, Debug, ImDraw)]
pub enum LockedPieceResult {
    Nothing,

    Single([u8; 1]),
    Double([u8; 2]),
    Triple([u8; 3]),
    Tetris([u8; 4]),

    MiniTSpin,
    MiniTSpinSingle,
    MiniTSpinDouble,
    TSpin, // ?
    TSpinSingle,
    TSpinDouble,
    TSpinTriple,
}

impl LockedPieceResult {
    pub fn get_lines_to_clear_slice<'a>(&'a self) -> &'a [u8] {
        match self {
            LockedPieceResult::Single(s) => s,
            LockedPieceResult::Double(s) => s,
            LockedPieceResult::Triple(s) => s,
            LockedPieceResult::Tetris(s) => s,
            _ => &[],
        }
    }
}

// T-Spin explanation and examples: http://harddrop.com/fumen/?m115@sgg0Aeg0QeAtAeAtreF811AyoSTASo78A2no2ACD5r?DlsCSASI/MESGNXEzoo2AJG98AQ51JEBD98AQo7aEJPONEO?BAAAvhGFcutAyoSTASoTABEoo2AUoo2Aw+kkDloo2ApN98A?Q5bkDJmZTASYlNE3CaoDTBAAAFcuxACD5rDFbcRATG88AwU?jXEuICbEFbMLEuoo2AiSg/DFbMLEmoo2AUoo2Aw+kkDFBAA?AFcueAzno2Aj3UNEyoSTASIPrDMj0TAS4wrDxQWXEFnBAAF?curAmXyTAS4wrDnAFeDyoo2AUEzPEJG98AwWyTASo93Du+8?8AQemsCwOxCAFcuzAyno2AyoSTASIPrDMj0TAS4wrDxQWXE?F388AQZjXEFbUVEl7gDEBM98AQemsCwOxCAFcusAV2krDzN?98AQemsCwOpTASYlWEJ5krDFbs9DpViTASIE2DplbTASosa?EFcu0Ayno2AynNbEFbEmDvjpTASo78A2no2Axno2Aj3khEN?G98AQurTASY91Dloo2AzuSrDsggHAegHQeAPAeAPreAAtjA?yYZhEsCyTASYttAzI2JEFbEBEJGVTASI3CElCCbElsKBAMh?H8CeH8AeE8JedruAAMhgWQeAPAeAPNeFrfMhglQeAtAeAtN?edrfvhAFrfMhAPQegWAeAPNeNrfHhC8BeAtQeglAeAtNetl?fMhgWQeAPAeAPNeFrf3gB8IeA8GeA8AeglIeA8GeAtAeAtN?e1gfKhAPAegWQeAPAegWNedrfKhAtAeglQeAPAeglNe1gfK?hAPAegWSegWNedru2AP2EvEFb85AFbUVEF388Aw08CEsoo2?AiA3TASIbeEJzkTAS4wrDnAFeDyoo2AUEzPEJ2BAAvhAdru?lAiYwdD1NVTASICvDFbEwCtMN5Duoo2A0LmQEs488AwAukD?LBAAAKhAtAeglIeAAGeA8AeglNe1guAAvhBdrfFrfKhgWAe?APSeAPNeNrfKhglAeAtQeAAAeAtNe1gfvhBdrfFrfKhgWAe?APSeAPNeNrfhgB8IeA8FeB8AeA8BeE8BeB8AeD8AeBAgHA8?AtFeAAC8FeAAB8AtNeNQfvhC1VfdgfdlfAhgWQeAPAeAPZe?FlfhgBAFeB8AeAAGeAAAeGABeA8glIeA8GeAPA8AtZeVbf9?ggWAeAPQegWAeAPaetkfXgB8IeA8IeG8CeG8glA8APHeA8H?eglAeAPaeNLfvhA1QfqgAPAegWQeAPAegWtedbfXgBAIeAA?FeAAAPAegHFAAeB8GAAeAPAegHFAAeAAAeGAKeAAA8QeFgf?HhgWSegWAeAPQetpu2AP2EvEFb85AFbUVEF388Aw08CEsoo?2AiA3TASIbeEJzkTAS4wrDnAFeDyoo2AUEzPEJ2BAAvhAtp?ulAiYwdD1NVTASICvDFbEwCtMN5Duoo2A0LmQEs488AwAuk?DLBAAAkgB8IeA8FeB8AeD8AeC8CeC8glC8BeD8AeB8AeBAD?eglA8AtDAMeVXuQAmXyTASY91Dloo2As3cyEvhE9hf9rfVw?f1wfNrf3ggWAegWSeAPgelhuyAyYZhEsCyTASYttAzI2JEF?bcRASExrD2ICbEloo2AUEzPEJG98AQuR5DQDVTAylAAAkgB?AIeAACeDAgHAegHGACeGABeAPGABeGADeCAJeAAtgANOJ5D?FbEwCtMN5Duoo2AsOprDFbsiDs4DXEz4CwBkgB8IeA8CeE8?AeH8CeG8BeH8BeH8BeD8Je9huAA3ggWAegWSeAPgelhuYAP?2EvEFb8bDFbcYCJGeTASYttAzI2JE3gglAeglSeAtge9ruA?A3ggWAegWSeAPgelhuzAP2EvEFb8bDFbcYCJGeTASYttAzI?2JEFbcRASEYNEFbEwCyuVDEloo2Areg/DFr4AAvhBlhu4A0?LmQEs488AwAukDr4CwBFbU9AFbUVEvz0TASYBNEXmbfEwow?2BFb85AFbEcEvoo2AzuSrDlhu9Ayno2AynNbEFbEmDvjZ1A?VJ98AQo78AQurTASIT5Dk488Aw3K6BFb0HEvoo2A0LmQEs4?88AwAukDLBAAAkgBAIeAAFeAAgHAegHCAAeCACeCABeBABe?APCABeBAB8FeAAAeB8NeFguAAIhgWSegWAeAPPeNquYAP2E?vEFb8bDFbcYCJGeTASYttAzI2JEzgCAAeF8AAEeD8AeglB8?AeE8BeA8AeAAFeglA8APAAOe1guAAvhBdrfFhfKhAPAegWS?egWNedruYAP2EvEFb8bDFbcYCJGeTASYttAzI2JE3gFAFeG?AAPAeglIeAAGeA8AeglNedrujAtnceEFb0sDy4vhEF22TAS?o93Du+88AQemsCwO5aElsKBAMhAPQegWAegWNeVrfMhAPDA?EeA8GegHAegHNetqfWhAPXeVwflhC8g0A8g0D8VwujAFbew?DyHRKEkoo2AjHRKE0N98AwR0TAS414DMD9nDFr4AAHhC8Le?APDADeB8DeCAgHAAgHDAVrujAtnceEFb0sDy4vhEF22TASo?93Du+88AQemsCwO5aElsKBAJhgWSegWAeAPOetqfHhBAgHG?eCAGeAAAeglA8AtOe1puYABjkNEFbMmEPsyaEFbMLEuoo2A?3iMDEchAPReNpupAFbemEBzkTASI/MEV2GbEFb8bDzoo2Am?uMDEF388Aw08CEsN98AZAAAA/gB8HeA8IeA8GeA8AtAeAAP?eVrueABjkNEFbMmEPsyaEFbMmEJ/bTASIClEF87dDurBAAJ?hgWSegWAeAPOetqf

#[derive(Copy, Clone, Debug, ImDraw)]
pub enum LastPieceAction {
    Movement,
    // @TODO we will need to know if the piece has wall kicked or not, and which kick, for T-Spins
    Rotation,
}

#[derive(Copy, Clone, Debug, ImDraw)]
pub struct LockedPiece {
    pub piece: Piece,
    pub pos: Vec2i,
    pub soft_drop_steps: u8,
    pub hard_drop_steps: u8,
    pub last_piece_action: LastPieceAction,
    pub lock_piece_result: LockedPieceResult,
}

pub fn lock_piece(
    piece: &Piece,
    pos: Vec2i,
    playfield: &mut Playfield,
) {
    for block_pos in piece.blocks() {
        playfield.set_block(
            pos.x + block_pos.x,
            pos.y + block_pos.y,
            piece.variant,
        );
    }
}

pub fn is_piece_locking(
    piece: &Piece,
    pos: Vec2i,
    playfield: &Playfield,
) -> bool {
    for block_pos in piece.blocks() {
        let down_x = pos.x + block_pos.x;
        let down_y = pos.y + block_pos.y - 1;
        if playfield.block(down_x, down_y).is_some() {
            return true;
        }
    }

    false
}

