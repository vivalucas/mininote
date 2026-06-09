#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Locale {
    #[default]
    ZhCn,
    EnUs,
    ZhTw,
    Ja,
    Ko,
    De,
    Fr,
    Es,
    PtBr,
    It,
    Ru,
}

impl Locale {
    pub fn from_tag(value: &str) -> Self {
        match value.trim().to_ascii_lowercase().as_str() {
            "en-us" | "en" => Self::EnUs,
            "zh-hk" | "zh-tw" | "zh-hant" => Self::ZhTw,
            "ja" | "ja-jp" => Self::Ja,
            "ko" | "ko-kr" => Self::Ko,
            "de" | "de-de" => Self::De,
            "fr" | "fr-fr" => Self::Fr,
            "es" | "es-es" => Self::Es,
            "pt" | "pt-br" | "pt-bz" => Self::PtBr,
            "it" | "it-it" => Self::It,
            "ru" | "ru-ru" => Self::Ru,
            _ => Self::ZhCn,
        }
    }
}

fn localized(
    locale: Locale,
    zh_cn: &'static str,
    en_us: &'static str,
    zh_tw: &'static str,
    ja: &'static str,
    ko: &'static str,
    de: &'static str,
    fr: &'static str,
    es: &'static str,
    pt_br: &'static str,
    it: &'static str,
    ru: &'static str,
) -> &'static str {
    match locale {
        Locale::ZhCn => zh_cn,
        Locale::EnUs => en_us,
        Locale::ZhTw => zh_tw,
        Locale::Ja => ja,
        Locale::Ko => ko,
        Locale::De => de,
        Locale::Fr => fr,
        Locale::Es => es,
        Locale::PtBr => pt_br,
        Locale::It => it,
        Locale::Ru => ru,
    }
}

pub fn app_name(_locale: Locale) -> &'static str {
    "MiniNote"
}

pub fn main_window_title(locale: Locale) -> &'static str {
    app_name(locale)
}

pub fn notepad_window_title(locale: Locale) -> &'static str {
    localized(
        locale,
        "MiniNote便签",
        "MiniNote Quick Note",
        "MiniNote便箋",
        "MiniNote クイックノート",
        "MiniNote 빠른 메모",
        "MiniNote Kurznotiz",
        "MiniNote Note rapide",
        "MiniNote Nota rápida",
        "MiniNote Nota rápida",
        "MiniNote Nota rapida",
        "MiniNote Быстрая заметка",
    )
}

pub fn tile_window_title(locale: Locale) -> &'static str {
    localized(
        locale,
        "MiniNote磁贴",
        "MiniNote Pin Mode",
        "MiniNote磁貼",
        "MiniNote 固定モード",
        "MiniNote 고정 모드",
        "MiniNote Anheften",
        "MiniNote Épinglé",
        "MiniNote Fijado",
        "MiniNote Fixado",
        "MiniNote Fissato",
        "MiniNote Закреплено",
    )
}

pub fn tray_tooltip(locale: Locale) -> &'static str {
    app_name(locale)
}

pub fn tray_show_main_label(locale: Locale) -> &'static str {
    localized(
        locale,
        "打开主窗口",
        "Open Main Window",
        "打開主視窗",
        "メインウィンドウを開く",
        "메인 창 열기",
        "Hauptfenster öffnen",
        "Ouvrir la fenêtre principale",
        "Abrir ventana principal",
        "Abrir janela principal",
        "Apri finestra principale",
        "Открыть главное окно",
    )
}

pub fn tray_quick_note_label(locale: Locale) -> &'static str {
    localized(
        locale,
        "快速记录",
        "Quick Note",
        "快速便箋",
        "クイックノート",
        "빠른 메모",
        "Kurznotiz",
        "Note rapide",
        "Nota rápida",
        "Nota rápida",
        "Nota rapida",
        "Быстрая заметка",
    )
}

pub fn tray_toggle_close_to_tray_label(locale: Locale) -> &'static str {
    localized(
        locale,
        "关闭到托盘",
        "Close to Tray",
        "關閉到系統匣",
        "閉じてトレイへ",
        "닫을 때 트레이로",
        "In Tray schließen",
        "Fermer dans la barre d’état",
        "Cerrar a la bandeja",
        "Fechar para a bandeja",
        "Chiudi nella tray",
        "Сворачивать в трей",
    )
}

pub fn tray_toggle_autostart_label(locale: Locale) -> &'static str {
    localized(
        locale,
        "开机自启动",
        "Launch on Startup",
        "開機自啟",
        "ログイン時に起動",
        "시작 시 실행",
        "Beim Start öffnen",
        "Lancer au démarrage",
        "Iniciar al arrancar",
        "Iniciar com o sistema",
        "Avvia all'accesso",
        "Запуск при входе",
    )
}

pub fn tray_quit_label(locale: Locale) -> &'static str {
    localized(
        locale,
        "退出",
        "Quit",
        "退出",
        "終了",
        "종료",
        "Beenden",
        "Quitter",
        "Salir",
        "Sair",
        "Esci",
        "Выход",
    )
}

pub fn macos_menu_file_label(locale: Locale) -> &'static str {
    localized(
        locale,
        "文件",
        "File",
        "檔案",
        "ファイル",
        "파일",
        "Ablage",
        "Fichier",
        "Archivo",
        "Arquivo",
        "File",
        "Файл",
    )
}

pub fn macos_menu_edit_label(locale: Locale) -> &'static str {
    localized(
        locale,
        "编辑",
        "Edit",
        "編輯",
        "編集",
        "편집",
        "Bearbeiten",
        "Édition",
        "Editar",
        "Editar",
        "Modifica",
        "Правка",
    )
}

pub fn macos_menu_view_label(locale: Locale) -> &'static str {
    localized(
        locale,
        "显示",
        "View",
        "顯示",
        "表示",
        "보기",
        "Darstellung",
        "Présentation",
        "Ver",
        "Visualizar",
        "Vista",
        "Вид",
    )
}

pub fn macos_menu_window_label(locale: Locale) -> &'static str {
    localized(
        locale,
        "窗口",
        "Window",
        "視窗",
        "ウィンドウ",
        "윈도우",
        "Fenster",
        "Fenêtre",
        "Ventana",
        "Janela",
        "Finestra",
        "Окно",
    )
}

pub fn macos_menu_help_label(locale: Locale) -> &'static str {
    localized(
        locale,
        "帮助",
        "Help",
        "幫助",
        "ヘルプ",
        "도움말",
        "Hilfe",
        "Aide",
        "Ayuda",
        "Ajuda",
        "Aiuto",
        "Справка",
    )
}

pub fn macos_menu_about_label(locale: Locale) -> String {
    match locale {
        Locale::ZhCn => format!("关于{}", app_name(locale)),
        Locale::EnUs => format!("About {}", app_name(locale)),
        Locale::ZhTw => format!("關於{}", app_name(locale)),
        Locale::Ja => format!("{}について", app_name(locale)),
        Locale::Ko => format!("{} 정보", app_name(locale)),
        Locale::De => format!("Über {}", app_name(locale)),
        Locale::Fr => format!("À propos de {}", app_name(locale)),
        Locale::Es => format!("Acerca de {}", app_name(locale)),
        Locale::PtBr => format!("Sobre o {}", app_name(locale)),
        Locale::It => format!("Informazioni su {}", app_name(locale)),
        Locale::Ru => format!("О {}", app_name(locale)),
    }
}

pub fn macos_menu_services_label(locale: Locale) -> &'static str {
    localized(
        locale,
        "服务",
        "Services",
        "服務",
        "サービス",
        "서비스",
        "Dienste",
        "Services",
        "Servicios",
        "Serviços",
        "Servizi",
        "Службы",
    )
}

pub fn macos_menu_hide_app_label(locale: Locale) -> String {
    match locale {
        Locale::ZhCn => format!("隐藏{}", app_name(locale)),
        Locale::EnUs => format!("Hide {}", app_name(locale)),
        Locale::ZhTw => format!("隱藏{}", app_name(locale)),
        Locale::Ja => format!("{}を隠す", app_name(locale)),
        Locale::Ko => format!("{} 가리기", app_name(locale)),
        Locale::De => format!("{} ausblenden", app_name(locale)),
        Locale::Fr => format!("Masquer {}", app_name(locale)),
        Locale::Es => format!("Ocultar {}", app_name(locale)),
        Locale::PtBr => format!("Ocultar {}", app_name(locale)),
        Locale::It => format!("Nascondi {}", app_name(locale)),
        Locale::Ru => format!("Скрыть {}", app_name(locale)),
    }
}

pub fn macos_menu_hide_others_label(locale: Locale) -> &'static str {
    localized(
        locale,
        "隐藏其他",
        "Hide Others",
        "隱藏其他",
        "ほかを隠す",
        "다른 항목 가리기",
        "Andere ausblenden",
        "Masquer les autres",
        "Ocultar otros",
        "Ocultar outros",
        "Nascondi altri",
        "Скрыть остальные",
    )
}

pub fn macos_menu_quit_app_label(locale: Locale) -> String {
    match locale {
        Locale::ZhCn => format!("退出{}", app_name(locale)),
        Locale::EnUs => format!("Quit {}", app_name(locale)),
        Locale::ZhTw => format!("退出{}", app_name(locale)),
        Locale::Ja => format!("{}を終了", app_name(locale)),
        Locale::Ko => format!("{} 종료", app_name(locale)),
        Locale::De => format!("{} beenden", app_name(locale)),
        Locale::Fr => format!("Quitter {}", app_name(locale)),
        Locale::Es => format!("Salir de {}", app_name(locale)),
        Locale::PtBr => format!("Sair do {}", app_name(locale)),
        Locale::It => format!("Esci da {}", app_name(locale)),
        Locale::Ru => format!("Завершить {}", app_name(locale)),
    }
}

pub fn macos_menu_close_window_label(locale: Locale) -> &'static str {
    localized(
        locale,
        "关闭窗口",
        "Close Window",
        "關閉視窗",
        "ウィンドウを閉じる",
        "윈도우 닫기",
        "Fenster schließen",
        "Fermer la fenêtre",
        "Cerrar ventana",
        "Fechar janela",
        "Chiudi finestra",
        "Закрыть окно",
    )
}

pub fn macos_menu_minimize_label(locale: Locale) -> &'static str {
    localized(
        locale,
        "最小化",
        "Minimize",
        "最小化",
        "最小化",
        "최소화",
        "Minimieren",
        "Réduire",
        "Minimizar",
        "Minimizar",
        "Riduci a icona",
        "Свернуть",
    )
}

pub fn macos_menu_zoom_label(locale: Locale) -> &'static str {
    localized(
        locale,
        "缩放",
        "Zoom",
        "縮放",
        "拡大/縮小",
        "확대/축소",
        "Zoomen",
        "Zoom",
        "Zoom",
        "Zoom",
        "Zoom",
        "Масштаб",
    )
}

pub fn macos_menu_fullscreen_label(locale: Locale) -> &'static str {
    localized(
        locale,
        "进入全屏",
        "Enter Full Screen",
        "進入全螢幕",
        "フルスクリーンにする",
        "전체 화면 시작",
        "Vollbild ein",
        "Passer en plein écran",
        "Entrar en pantalla completa",
        "Entrar em tela cheia",
        "Vai a schermo intero",
        "На весь экран",
    )
}

pub fn macos_menu_undo_label(locale: Locale) -> &'static str {
    localized(
        locale,
        "撤销",
        "Undo",
        "復原",
        "取り消す",
        "실행 취소",
        "Widerrufen",
        "Annuler",
        "Deshacer",
        "Desfazer",
        "Annulla",
        "Отменить",
    )
}

pub fn macos_menu_redo_label(locale: Locale) -> &'static str {
    localized(
        locale,
        "重做",
        "Redo",
        "重做",
        "やり直す",
        "다시 실행",
        "Wiederholen",
        "Rétablir",
        "Rehacer",
        "Refazer",
        "Ripeti",
        "Повторить",
    )
}

pub fn macos_menu_cut_label(locale: Locale) -> &'static str {
    localized(
        locale,
        "剪切",
        "Cut",
        "剪下",
        "カット",
        "오려두기",
        "Ausschneiden",
        "Couper",
        "Cortar",
        "Recortar",
        "Taglia",
        "Вырезать",
    )
}

pub fn macos_menu_copy_label(locale: Locale) -> &'static str {
    localized(
        locale,
        "复制",
        "Copy",
        "複製",
        "コピー",
        "복사",
        "Kopieren",
        "Copier",
        "Copiar",
        "Copiar",
        "Copia",
        "Копировать",
    )
}

pub fn macos_menu_paste_label(locale: Locale) -> &'static str {
    localized(
        locale,
        "粘贴",
        "Paste",
        "貼上",
        "ペースト",
        "붙여넣기",
        "Einsetzen",
        "Coller",
        "Pegar",
        "Colar",
        "Incolla",
        "Вставить",
    )
}

pub fn macos_menu_select_all_label(locale: Locale) -> &'static str {
    localized(
        locale,
        "全选",
        "Select All",
        "全選",
        "すべてを選択",
        "모두 선택",
        "Alles auswählen",
        "Tout sélectionner",
        "Seleccionar todo",
        "Selecionar tudo",
        "Seleziona tutto",
        "Выбрать все",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_supported_locales_and_falls_back_to_source_locale() {
        assert_eq!(Locale::from_tag("zh-CN"), Locale::ZhCn);
        assert_eq!(Locale::from_tag("en-US"), Locale::EnUs);
        assert_eq!(Locale::from_tag("zh-HK"), Locale::ZhTw);
        assert_eq!(Locale::from_tag("zh-TW"), Locale::ZhTw);
        assert_eq!(Locale::from_tag("ja-JP"), Locale::Ja);
        assert_eq!(Locale::from_tag("ko-KR"), Locale::Ko);
        assert_eq!(Locale::from_tag("de-DE"), Locale::De);
        assert_eq!(Locale::from_tag("fr-FR"), Locale::Fr);
        assert_eq!(Locale::from_tag("es-ES"), Locale::Es);
        assert_eq!(Locale::from_tag("pt-BR"), Locale::PtBr);
        assert_eq!(Locale::from_tag("it-IT"), Locale::It);
        assert_eq!(Locale::from_tag("ru-RU"), Locale::Ru);
        assert_eq!(Locale::from_tag("nl-NL"), Locale::ZhCn);
    }

    #[test]
    fn localizes_native_shell_strings_for_supported_locales() {
        assert_eq!(app_name(Locale::ZhCn), "MiniNote");
        assert_eq!(notepad_window_title(Locale::EnUs), "MiniNote Quick Note");
        assert_eq!(tile_window_title(Locale::ZhTw), "MiniNote磁貼");
        assert_eq!(tray_tooltip(Locale::Ja), "MiniNote");
        assert_eq!(tray_show_main_label(Locale::De), "Hauptfenster öffnen");
        assert_eq!(tray_quick_note_label(Locale::Ko), "빠른 메모");
        assert_eq!(
            tray_toggle_close_to_tray_label(Locale::Fr),
            "Fermer dans la barre d’état"
        );
        assert_eq!(
            tray_toggle_autostart_label(Locale::PtBr),
            "Iniciar com o sistema"
        );
        assert_eq!(tray_quit_label(Locale::Ru), "Выход");
    }

    #[test]
    fn localizes_macos_native_menu_strings_for_supported_locales() {
        assert_eq!(macos_menu_file_label(Locale::ZhCn), "文件");
        assert_eq!(macos_menu_edit_label(Locale::ZhTw), "編輯");
        assert_eq!(macos_menu_view_label(Locale::EnUs), "View");
        assert_eq!(macos_menu_window_label(Locale::Ja), "ウィンドウ");
        assert_eq!(macos_menu_help_label(Locale::Es), "Ayuda");
        assert_eq!(macos_menu_about_label(Locale::De), "Über MiniNote");
        assert_eq!(macos_menu_services_label(Locale::It), "Servizi");
        assert_eq!(macos_menu_hide_app_label(Locale::Fr), "Masquer MiniNote");
        assert_eq!(macos_menu_hide_others_label(Locale::Ru), "Скрыть остальные");
        assert_eq!(macos_menu_quit_app_label(Locale::PtBr), "Sair do MiniNote");
        assert_eq!(macos_menu_close_window_label(Locale::Ko), "윈도우 닫기");
        assert_eq!(macos_menu_minimize_label(Locale::EnUs), "Minimize");
        assert_eq!(macos_menu_zoom_label(Locale::ZhCn), "缩放");
        assert_eq!(macos_menu_fullscreen_label(Locale::ZhTw), "進入全螢幕");
        assert_eq!(macos_menu_undo_label(Locale::ZhTw), "復原");
        assert_eq!(macos_menu_redo_label(Locale::ZhCn), "重做");
        assert_eq!(macos_menu_cut_label(Locale::ZhTw), "剪下");
        assert_eq!(macos_menu_copy_label(Locale::ZhCn), "复制");
        assert_eq!(macos_menu_paste_label(Locale::EnUs), "Paste");
        assert_eq!(macos_menu_select_all_label(Locale::ZhTw), "全選");
    }
}
