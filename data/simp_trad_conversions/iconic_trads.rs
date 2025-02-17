use lazy_static::lazy_static;
use std::collections::HashSet;

lazy_static! {
    pub static ref ICONIC_TRADS: HashSet<char> = {
        HashSet::from([
            '丟', '並', '亂', '亙', '亞', '佇', '佈', '佔', '併', '來', '侖', '侶', '侷', '俁',
            '係', '俠', '倆', '倈', '倉', '個', '們', '倖', '倫', '偉', '側', '偵', '偽', '傑',
            '傖', '傘', '備', '傢', '傭', '傯', '傳', '傴', '債', '傷', '傾', '僂', '僅', '僉',
            '僑', '僕', '僞', '僥', '僱', '價', '儀', '儂', '億', '儈', '儉', '儐', '儔', '儕',
            '儘', '償', '優', '儲', '儷', '儸', '儺', '儻', '儼', '兇', '兌', '兒', '兗', '內',
            '兩', '冊', '冪', '凍', '凜', '凱', '別', '刪', '剄', '則', '剎', '剛', '剝', '剮',
            '剴', '創', '剷', '劃', '劇', '劉', '劊', '劌', '劍', '劑', '勁', '動', '務', '勛',
            '勝', '勞', '勢', '勣', '勱', '勳', '勵', '勸', '勻', '匭', '匯', '匱', '區', '協',
            '卹', '卻', '厙', '厠', '厤', '厭', '厲', '參', '叄', '叢', '吒', '吳', '吶', '呂',
            '員', '唄', '唸', '問', '啓', '啞', '啟', '啢', '喚', '喪', '喫', '喬', '單', '喲',
            '嗆', '嗇', '嗎', '嗚', '嗩', '嗶', '嘆', '嘍', '嘔', '嘖', '嘗', '嘜', '嘩', '嘮',
            '嘯', '嘰', '嘸', '噁', '噓', '噝', '噠', '噥', '噯', '噲', '噴', '噸', '噹', '嚀',
            '嚇', '嚕', '嚙', '嚥', '嚦', '嚨', '嚮', '嚳', '嚴', '嚶', '囀', '囂', '囈', '囉',
            '囑', '囪', '圇', '國', '圍', '園', '圓', '圖', '團', '埡', '埰', '執', '堅', '堊',
            '堝', '堯', '報', '場', '塊', '塋', '塏', '塗', '塚', '塢', '塤', '塵', '塹', '墊',
            '墜', '墮', '墳', '墻', '墾', '壇', '壋', '壎', '壓', '壘', '壙', '壞', '壟', '壠',
            '壢', '壩', '壯', '壺', '壼', '壽', '夠', '夢', '夾', '奐', '奧', '奩', '奪', '奬',
            '奮', '妝', '姍', '姦', '娛', '婁', '婦', '婭', '媧', '媯', '媼', '媽', '嫋', '嫗',
            '嫵', '嫺', '嫻', '嬀', '嬈', '嬋', '嬌', '嬙', '嬡', '嬤', '嬪', '嬰', '嬸', '孃',
            '孌', '孫', '學', '孿', '宮', '寢', '實', '寧', '審', '寫', '寬', '寵', '寶', '將',
            '專', '尋', '對', '導', '尷', '屆', '屍', '屜', '屢', '層', '屬', '岡', '峴', '島',
            '峽', '崍', '崑', '崗', '崙', '崢', '嵐', '嵗', '嶄', '嶇', '嶔', '嶗', '嶠', '嶢',
            '嶧', '嶸', '嶺', '嶼', '嶽', '巋', '巒', '巔', '巖', '巗', '巰', '巹', '帥', '師',
            '帳', '帶', '幀', '幃', '幗', '幘', '幟', '幣', '幫', '幬', '幹', '幾', '庫', '廁',
            '廂', '廄', '廈', '廎', '廕', '廚', '廝', '廟', '廠', '廡', '廢', '廣', '廩', '廬',
            '廳', '弒', '弔', '弳', '張', '強', '彈', '彌', '彎', '彙', '彠', '彥', '彫', '彿',
            '後', '徑', '從', '徠', '復', '徹', '恆', '恥', '悅', '悵', '悶', '悽', '惡', '惱',
            '惲', '惻', '愛', '愜', '愨', '愴', '愷', '愾', '慄', '態', '慍', '慘', '慚', '慟',
            '慣', '慤', '慫', '慮', '慳', '慶', '慾', '憂', '憊', '憐', '憑', '憒', '憚', '憤',
            '憫', '憮', '憲', '憶', '懇', '應', '懌', '懍', '懟', '懣', '懲', '懶', '懷', '懸',
            '懺', '懼', '懾', '戀', '戇', '戔', '戧', '戩', '戰', '戲', '戶', '拋', '挾', '捨',
            '捫', '捲', '掃', '掄', '掙', '掛', '採', '揀', '揚', '換', '揮', '損', '搖', '搗',
            '搶', '摑', '摜', '摟', '摯', '摳', '摶', '摺', '摻', '撈', '撐', '撓', '撟', '撣',
            '撥', '撫', '撲', '撳', '撻', '撾', '撿', '擁', '擄', '擇', '擊', '擋', '擔', '據',
            '擠', '擣', '擬', '擯', '擰', '擱', '擲', '擴', '擷', '擺', '擻', '擼', '擾', '攄',
            '攆', '攏', '攔', '攖', '攙', '攛', '攜', '攝', '攢', '攣', '攤', '攪', '攬', '敗',
            '敘', '敵', '數', '斂', '斃', '斕', '斬', '斷', '旂', '昇', '時', '晉', '晝', '暈',
            '暉', '暘', '暢', '暫', '曄', '曆', '曇', '曉', '曖', '曠', '曬', '書', '會', '朧',
            '朮', '東', '柵', '桿', '梔', '梘', '條', '梟', '棄', '棖', '棗', '棟', '棧', '棲',
            '棶', '椏', '楊', '楓', '楨', '業', '極', '榦', '榮', '榿', '構', '槍', '槓', '槤',
            '槨', '槮', '槳', '樁', '樂', '樅', '樑', '樓', '標', '樞', '樣', '樸', '樹', '樺',
            '橈', '橋', '機', '橢', '橫', '檁', '檉', '檔', '檜', '檟', '檢', '檣', '檮', '檯',
            '檳', '檸', '檻', '櫃', '櫓', '櫚', '櫛', '櫝', '櫞', '櫟', '櫥', '櫨', '櫪', '櫫',
            '櫬', '櫳', '櫸', '櫻', '欄', '權', '欏', '欒', '欖', '欽', '歎', '歐', '歟', '歡',
            '歲', '歷', '歸', '歿', '殘', '殞', '殤', '殫', '殭', '殮', '殯', '殲', '殺', '殻',
            '殼', '毀', '毆', '毿', '氂', '氈', '氣', '氫', '氬', '氳', '氾', '汎', '汙', '決',
            '沒', '沖', '況', '泝', '洩', '洶', '浹', '涇', '涼', '淒', '淚', '淥', '淨', '淩',
            '淪', '淵', '淶', '淺', '渙', '減', '渢', '渦', '測', '渾', '湊', '湞', '湧', '湯',
            '溈', '準', '溝', '溫', '溮', '溼', '滄', '滅', '滌', '滎', '滙', '滬', '滯', '滲',
            '滷', '滸', '滻', '滾', '滿', '漁', '漊', '漚', '漢', '漣', '漬', '漲', '漵', '漸',
            '漿', '潁', '潑', '潔', '潛', '潤', '潯', '潰', '潿', '澀', '澆', '澇', '澗', '澠',
            '澤', '澮', '澱', '濁', '濃', '濕', '濘', '濛', '濟', '濤', '濫', '濰', '濱', '濺',
            '濼', '濾', '瀅', '瀆', '瀉', '瀋', '瀏', '瀕', '瀘', '瀝', '瀟', '瀠', '瀦', '瀧',
            '瀨', '瀰', '瀲', '瀾', '灃', '灑', '灕', '灘', '灝', '灣', '灤', '灩', '災', '為',
            '烏', '烴', '無', '煇', '煉', '煒', '煙', '煢', '煥', '煩', '煬', '熅', '熒', '熗',
            '熱', '熲', '熾', '燁', '燈', '燉', '燒', '燙', '燜', '營', '燦', '燬', '燭', '燴',
            '燻', '燼', '燾', '爍', '爐', '爛', '爭', '爲', '爺', '爾', '牀', '牆', '牘', '牽',
            '犖', '犛', '犢', '犧', '狀', '狹', '狽', '猙', '猶', '猻', '獁', '獃', '獄', '獅',
            '獎', '獨', '獪', '獫', '獮', '獰', '獲', '獵', '獷', '獸', '獺', '獻', '獼', '玀',
            '現', '琱', '琺', '琿', '瑋', '瑒', '瑣', '瑤', '瑩', '瑪', '瑲', '璉', '璡', '璣',
            '璦', '璫', '環', '璵', '璽', '璿', '瓊', '瓏', '瓔', '瓚', '甌', '甕', '產', '産',
            '甦', '甯', '畝', '畢', '畫', '異', '當', '疇', '疊', '痙', '痠', '瘋', '瘍', '瘓',
            '瘞', '瘡', '瘧', '瘺', '瘻', '療', '癆', '癇', '癉', '癒', '癘', '癟', '癡', '癢',
            '癤', '癥', '癩', '癬', '癭', '癮', '癰', '癱', '癲', '發', '皚', '皰', '皸', '皺',
            '盃', '盜', '盞', '盡', '監', '盤', '盧', '盪', '眥', '眾', '睜', '睞', '瞞', '瞼',
            '矇', '矚', '矯', '硃', '硜', '硤', '硨', '硯', '碩', '碭', '碸', '確', '碼', '磑',
            '磚', '磧', '磯', '磽', '礄', '礎', '礙', '礦', '礪', '礫', '礬', '礮', '礱', '祕',
            '祿', '禍', '禎', '禕', '禦', '禪', '禮', '禰', '禱', '禿', '秈', '稅', '稈', '稜',
            '稟', '種', '稱', '穀', '穌', '積', '穎', '穠', '穡', '穢', '穩', '穫', '窩', '窪',
            '窮', '窯', '窵', '窶', '窺', '竄', '竅', '竇', '竈', '竊', '竪', '競', '筆', '筍',
            '筧', '箇', '箋', '箏', '節', '範', '築', '篋', '篔', '篠', '篤', '篩', '篳', '簀',
            '簍', '簑', '簞', '簡', '簣', '簫', '簹', '簽', '簾', '籃', '籌', '籙', '籛', '籜',
            '籟', '籠', '籤', '籩', '籬', '籮', '籲', '粵', '糞', '糧', '糰', '糲', '糴', '糶',
            '糾', '紀', '紂', '約', '紅', '紆', '紇', '紈', '紉', '紋', '納', '紐', '紓', '純',
            '紕', '紗', '紘', '紙', '級', '紛', '紜', '紝', '紡', '紮', '細', '紱', '紲', '紳',
            '紵', '紹', '紺', '紿', '絀', '終', '絃', '組', '絆', '絎', '結', '絕', '絛', '絝',
            '絞', '絡', '絢', '給', '絨', '絰', '統', '絲', '絳', '絶', '絹', '綁', '綃', '綆',
            '綉', '綏', '綑', '經', '綜', '綞', '綠', '綢', '綣', '綫', '綬', '維', '綯', '綰',
            '綱', '網', '綳', '綴', '綵', '綸', '綹', '綺', '綻', '綽', '綾', '綿', '緄', '緇',
            '緊', '緋', '緑', '緒', '緗', '緘', '緙', '線', '緝', '緞', '締', '緡', '緣', '緦',
            '編', '緩', '緬', '緯', '緱', '緲', '練', '緹', '緻', '縈', '縉', '縊', '縋', '縐',
            '縑', '縛', '縝', '縞', '縟', '縣', '縧', '縫', '縭', '縮', '縱', '縲', '縴', '縵',
            '縶', '縷', '縹', '總', '績', '繃', '繅', '繆', '繒', '織', '繕', '繚', '繞', '繡',
            '繩', '繪', '繫', '繭', '繮', '繯', '繳', '繹', '繼', '繽', '繾', '纇', '纈', '纊',
            '續', '纍', '纏', '纓', '纔', '纖', '纘', '纜', '缽', '罈', '罌', '罎', '罰', '罵',
            '罷', '羅', '羆', '羈', '羋', '羥', '羨', '義', '羶', '習', '翬', '翹', '翽', '耬',
            '聖', '聞', '聯', '聰', '聲', '聳', '聵', '聶', '職', '聹', '聽', '聾', '肅', '脅',
            '脈', '脛', '脣', '脩', '脫', '脹', '腎', '腖', '腦', '腫', '腳', '腸', '膚', '膠',
            '膩', '膽', '膾', '膿', '臉', '臍', '臏', '臘', '臚', '臟', '臠', '臥', '臨', '臺',
            '與', '興', '舉', '舊', '舘', '艙', '艤', '艦', '艫', '艱', '艷', '芻', '苧', '茲',
            '荊', '莊', '莖', '莢', '莧', '華', '菴', '菸', '萇', '萊', '萬', '萵', '葉', '葒',
            '葦', '葯', '葷', '蒐', '蒓', '蒔', '蒞', '蒼', '蓀', '蓆', '蓋', '蓧', '蓮', '蓯',
            '蓴', '蓽', '蔔', '蔞', '蔣', '蔥', '蔦', '蔭', '蕁', '蕎', '蕒', '蕕', '蕘', '蕢',
            '蕩', '蕪', '蕭', '蕷', '薀', '薈', '薊', '薌', '薑', '薔', '薘', '薟', '薦', '薩',
            '薺', '藍', '藎', '藝', '藥', '藪', '藴', '藶', '藹', '藺', '蘄', '蘆', '蘇', '蘊',
            '蘋', '蘚', '蘞', '蘢', '蘭', '蘿', '處', '虛', '虜', '號', '虧', '虯', '蛺', '蛻',
            '蜆', '蝕', '蝟', '蝦', '蝨', '蝸', '螄', '螞', '螢', '螻', '螿', '蟄', '蟈', '蟎',
            '蟬', '蟯', '蟲', '蟶', '蟻', '蠅', '蠆', '蠍', '蠐', '蠑', '蠔', '蠟', '蠣', '蠱',
            '蠶', '蠻', '衆', '衊', '術', '衕', '衚', '衛', '衝', '袞', '裏', '補', '裝', '裡',
            '製', '複', '褌', '褘', '褲', '褳', '褸', '褻', '襖', '襠', '襤', '襪', '襬', '襯',
            '襲', '襴', '覈', '見', '規', '覓', '視', '覘', '覡', '覥', '覦', '親', '覬', '覯',
            '覲', '覷', '覺', '覽', '觀', '觴', '觶', '觸', '訂', '訃', '計', '訊', '訌', '討',
            '訐', '訓', '訕', '訖', '託', '記', '訛', '訝', '訟', '訢', '訣', '訥', '訪', '設',
            '許', '訴', '訶', '診', '註', '証', '詁', '詆', '詎', '詐', '詒', '詔', '評', '詗',
            '詘', '詛', '詞', '詠', '詡', '詢', '詣', '試', '詩', '詫', '詬', '詭', '詮', '詰',
            '話', '該', '詳', '詵', '詼', '詿', '誄', '誅', '誆', '誇', '誌', '認', '誑', '誒',
            '誕', '誘', '誚', '語', '誠', '誡', '誣', '誤', '誥', '誦', '誨', '說', '説', '誰',
            '課', '誹', '誼', '調', '諂', '諄', '談', '諉', '請', '諍', '諏', '諒', '論', '諗',
            '諛', '諜', '諡', '諢', '諤', '諦', '諧', '諫', '諭', '諮', '諱', '諳', '諶', '諷',
            '諸', '諺', '諾', '謀', '謁', '謂', '謄', '謊', '謎', '謐', '謔', '謖', '謗', '謙',
            '謚', '講', '謝', '謠', '謡', '謨', '謫', '謬', '謳', '謹', '謾', '譁', '證', '譎',
            '譏', '譖', '識', '譙', '譚', '譜', '譟', '譫', '譯', '議', '譴', '護', '譽', '讀',
            '變', '讌', '讎', '讒', '讓', '讕', '讖', '讚', '讜', '讞', '豈', '豎', '豐', '豔',
            '豬', '貓', '貝', '貞', '負', '財', '貢', '貧', '貨', '販', '貪', '貫', '責', '貯',
            '貰', '貲', '貳', '貴', '貶', '買', '貸', '貺', '費', '貼', '貽', '貿', '賀', '賁',
            '賂', '賃', '賄', '賅', '資', '賈', '賊', '賑', '賒', '賓', '賕', '賙', '賚', '賜',
            '賞', '賠', '賡', '賢', '賣', '賤', '賦', '賧', '質', '賬', '賭', '賴', '賺', '購',
            '賽', '賾', '贄', '贅', '贇', '贈', '贊', '贋', '贍', '贏', '贓', '贔', '贖', '贗',
            '贛', '赬', '趕', '趙', '趨', '跡', '踐', '踴', '蹌', '蹕', '蹟', '蹠', '蹣', '蹤',
            '蹺', '躂', '躉', '躊', '躋', '躍', '躑', '躒', '躕', '躡', '躥', '躪', '軀', '車',
            '軋', '軌', '軍', '軑', '軒', '軔', '軛', '軟', '軫', '軲', '軸', '軹', '軻', '軼',
            '軾', '較', '輅', '載', '輊', '輒', '輓', '輔', '輕', '輛', '輜', '輝', '輞', '輟',
            '輥', '輦', '輩', '輪', '輯', '輳', '輸', '輻', '輾', '輿', '轂', '轄', '轅', '轆',
            '轉', '轍', '轎', '轟', '轡', '轢', '轤', '辦', '辭', '辮', '辯', '農', '迴', '逕',
            '這', '連', '週', '進', '遊', '運', '過', '達', '違', '遙', '遜', '遞', '遠', '適',
            '遲', '遷', '選', '遺', '遼', '邁', '還', '邇', '邊', '邏', '郟', '郵', '鄆', '鄉',
            '鄒', '鄔', '鄖', '鄧', '鄭', '鄰', '鄲', '鄴', '鄶', '鄺', '酈', '醃', '醖', '醜',
            '醞', '醫', '醬', '醱', '釀', '釁', '釋', '釐', '釓', '釔', '釕', '釗', '釘', '釙',
            '針', '釣', '釤', '釦', '釧', '釩', '釵', '釷', '釹', '釺', '鈀', '鈁', '鈃', '鈄',
            '鈈', '鈉', '鈍', '鈎', '鈐', '鈑', '鈒', '鈔', '鈕', '鈞', '鈣', '鈥', '鈦', '鈧',
            '鈮', '鈰', '鈳', '鈴', '鈷', '鈸', '鈹', '鈺', '鈽', '鈾', '鈿', '鉀', '鉅', '鉈',
            '鉉', '鉋', '鉍', '鉑', '鉕', '鉗', '鉚', '鉛', '鉞', '鉢', '鉤', '鉦', '鉬', '鉭',
            '鉶', '鉸', '鉺', '鉻', '鉿', '銀', '銃', '銅', '銍', '銑', '銓', '銖', '銘', '銚',
            '銛', '銜', '銠', '銣', '銥', '銦', '銨', '銩', '銪', '銫', '銬', '銳', '銷', '銹',
            '銻', '銼', '鋁', '鋂', '鋅', '鋇', '鋌', '鋏', '鋒', '鋙', '鋟', '鋤', '鋥', '鋦',
            '鋨', '鋪', '鋭', '鋮', '鋯', '鋰', '鋱', '鋶', '鋸', '鋼', '錁', '錄', '錇', '錐',
            '錒', '錕', '錘', '錚', '錛', '錟', '錠', '錡', '錢', '錦', '錨', '錩', '錫', '錮',
            '錯', '録', '錳', '錶', '錸', '錼', '鍀', '鍆', '鍇', '鍊', '鍋', '鍍', '鍔', '鍘',
            '鍚', '鍛', '鍠', '鍤', '鍥', '鍩', '鍬', '鍰', '鍵', '鍶', '鍺', '鍼', '鍾', '鎂',
            '鎄', '鎊', '鎔', '鎖', '鎘', '鎚', '鎛', '鎢', '鎣', '鎦', '鎧', '鎩', '鎬', '鎮',
            '鎰', '鎳', '鎵', '鎸', '鎿', '鏃', '鏇', '鏈', '鏌', '鏐', '鏑', '鏗', '鏘', '鏜',
            '鏝', '鏞', '鏟', '鏡', '鏢', '鏤', '鏵', '鏷', '鏹', '鏽', '鐃', '鐐', '鐒', '鐓',
            '鐔', '鐘', '鐙', '鐠', '鐦', '鐧', '鐨', '鐫', '鐮', '鐲', '鐳', '鐵', '鐸', '鐺',
            '鐿', '鑄', '鑊', '鑌', '鑑', '鑒', '鑔', '鑕', '鑞', '鑠', '鑣', '鑥', '鑪', '鑭',
            '鑰', '鑲', '鑷', '鑼', '鑽', '鑾', '鑿', '钂', '長', '門', '閂', '閃', '閆', '閈',
            '閉', '開', '閎', '閏', '閑', '閒', '間', '閔', '閘', '閡', '閣', '閤', '閥', '閨',
            '閩', '閫', '閬', '閭', '閱', '閲', '閶', '閹', '閻', '閼', '閽', '閾', '閿', '闆',
            '闈', '闊', '闋', '闌', '闍', '闐', '闒', '闓', '闔', '闕', '闖', '關', '闞', '闡',
            '闢', '闥', '陘', '陝', '陞', '陣', '陰', '陳', '陸', '陽', '隊', '階', '隕', '際',
            '隨', '險', '隱', '隴', '隸', '隻', '雋', '雖', '雙', '雛', '雜', '雞', '離', '難',
            '雲', '電', '霑', '霧', '霽', '靂', '靄', '靈', '靚', '靜', '靦', '靨', '鞏', '鞦',
            '鞽', '韁', '韃', '韆', '韋', '韌', '韓', '韙', '韜', '韝', '韞', '韻', '響', '頁',
            '頂', '頃', '項', '順', '頇', '須', '頊', '頌', '頎', '預', '頑', '頒', '頓', '頗',
            '領', '頜', '頡', '頤', '頦', '頫', '頭', '頰', '頲', '頴', '頷', '頸', '頹', '頻',
            '頽', '顆', '題', '額', '顎', '顏', '顒', '顓', '顔', '願', '顙', '顛', '類', '顥',
            '顧', '顫', '顬', '顯', '顰', '顱', '顳', '顴', '風', '颮', '颯', '颱', '颳', '颶',
            '颺', '颼', '飄', '飆', '飈', '飛', '飢', '飥', '飩', '飪', '飫', '飭', '飯', '飲',
            '飴', '飼', '飽', '飾', '餃', '餄', '餅', '餉', '養', '餌', '餎', '餑', '餒', '餓',
            '餕', '餘', '餚', '餛', '餜', '餞', '餡', '餧', '館', '餵', '餺', '餾', '餿', '饃',
            '饅', '饈', '饉', '饊', '饋', '饌', '饑', '饒', '饗', '饞', '饢', '馬', '馭', '馮',
            '馱', '馳', '馴', '駁', '駐', '駑', '駒', '駔', '駕', '駘', '駙', '駛', '駝', '駟',
            '駡', '駢', '駭', '駰', '駱', '駸', '駿', '騁', '騂', '騅', '騎', '騏', '騙', '騤',
            '騫', '騭', '騮', '騰', '騶', '騷', '騸', '騾', '驀', '驁', '驂', '驃', '驄', '驅',
            '驊', '驌', '驍', '驕', '驗', '驚', '驛', '驟', '驢', '驤', '驥', '驪', '骯', '髏',
            '髒', '體', '髕', '髖', '髮', '鬆', '鬍', '鬚', '鬢', '鬥', '鬧', '鬨', '鬩', '鬮',
            '鬱', '魎', '魘', '魚', '魛', '魢', '魨', '魯', '魴', '魷', '鮁', '鮃', '鮊', '鮋',
            '鮍', '鮎', '鮐', '鮑', '鮒', '鮓', '鮜', '鮞', '鮪', '鮫', '鮭', '鮮', '鮶', '鯀',
            '鯁', '鯇', '鯉', '鯊', '鯒', '鯔', '鯕', '鯖', '鯗', '鯛', '鯝', '鯡', '鯢', '鯤',
            '鯧', '鯨', '鯪', '鯰', '鯷', '鯽', '鯿', '鰂', '鰃', '鰈', '鰉', '鰌', '鰍', '鰏',
            '鰐', '鰒', '鰓', '鰜', '鰟', '鰣', '鰥', '鰨', '鰩', '鰭', '鰮', '鰱', '鰲', '鰳',
            '鰵', '鰷', '鰹', '鰺', '鰻', '鰼', '鰾', '鱂', '鱅', '鱈', '鱉', '鱒', '鱔', '鱖',
            '鱗', '鱘', '鱝', '鱟', '鱠', '鱣', '鱤', '鱧', '鱨', '鱭', '鱷', '鱸', '鱺', '鳥',
            '鳧', '鳩', '鳬', '鳳', '鳴', '鳶', '鴆', '鴇', '鴉', '鴒', '鴕', '鴛', '鴝', '鴞',
            '鴟', '鴣', '鴦', '鴨', '鴯', '鴰', '鴴', '鴻', '鴿', '鵂', '鵐', '鵑', '鵒', '鵓',
            '鵜', '鵝', '鵠', '鵡', '鵪', '鵬', '鵯', '鵰', '鵲', '鶇', '鶉', '鶓', '鶖', '鶘',
            '鶚', '鶡', '鶥', '鶩', '鶯', '鶲', '鶴', '鶹', '鶺', '鶻', '鶼', '鶿', '鷀', '鷁',
            '鷂', '鷄', '鷊', '鷓', '鷖', '鷗', '鷙', '鷚', '鷥', '鷦', '鷯', '鷲', '鷴', '鷸',
            '鷹', '鷺', '鸌', '鸏', '鸕', '鸚', '鸛', '鸝', '鸞', '鹵', '鹹', '鹼', '鹽', '麗',
            '麥', '麩', '麪', '麫', '麯', '麴', '麵', '麼', '黃', '黌', '點', '黨', '黴', '黷',
            '黽', '黿', '鼂', '鼉', '鼴', '齊', '齋', '齎', '齏', '齒', '齗', '齙', '齜', '齟',
            '齠', '齡', '齣', '齦', '齧', '齪', '齬', '齲', '齶', '齷', '龍', '龐', '龔', '龕',
            '龜',
        ])
    };
}
