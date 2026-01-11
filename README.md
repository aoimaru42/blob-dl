<p align="center">
    <a>
    <img alt="`blob-dl` Logo" src="../assets/blob-dl-logo-v1.png" height="150">
  </a>
</p>

<div align="center">
    
![Crates.io](https://img.shields.io/crates/d/blob-dl?color=%2325BE5D)
![GitHub](https://img.shields.io/badge/license-MIT-blue)
![GitHub issues](https://img.shields.io/github/issues/MicheleCioccarelli/blob-dl)
![Crates.io](https://img.shields.io/crates/v/blob-dl)
    
</div>

<h1 align="center">blob-dl</h1>

> **Note:** このリポジトリは [MicheleCioccarelli/blob-dl](https://github.com/MicheleCioccarelli/blob-dl) のフォークです。

`blob-dl` は、YouTubeから動画や音声ファイルをダウンロードするためのコマンドラインツールです。[`yt-dlp`](https://github.com/yt-dlp/yt-dlp) へのインターフェースとして機能し、一連の質問を通じて、あなたのニーズに合った `yt-dlp` コマンドを生成し、実行します。

このプログラムの目的は、`yt-dlp` に希望の動作をさせるために必要なフラグを調べるという面倒な作業をなくすことです。
`blob-dl` を使用する際は、ダウンロードしたいURLさえ知っていれば十分で、残りは自動的に処理されます。

- 詳細については、[機能](#機能) セクションを参照してください

[![asciicast](https://asciinema.org/a/jZUokSc5oDms6vICdNTic1vxh.svg)](https://asciinema.org/a/jZUokSc5oDms6vICdNTic1vxh)


# インストール
`blob-dl` をインストールする最も簡単な方法は、[バイナリファイル](https://github.com/MicheleCioccarelli/blob-dl/releases/) を使用することです

あるいは、Rustプログラマーの場合は、`cargo` を使用して `blob-dl` をインストールできます

```
$ cargo install blob-dl
```
## 依存関係
`blob-dl` は `yt-dlp` に依存しています。公式の[ガイド](https://github.com/yt-dlp/yt-dlp#installation)に従ってインストールしてください。

`blob-dl` のすべての機能にアクセスするには、`yt-dlp` の[推奨依存関係](https://github.com/yt-dlp/yt-dlp#dependencies)（特に `ffmpeg` と `ffprobe`）もインストールすることをお勧めします。

# 使用方法
`blob-dl` を使用するには、ダウンロードしたい動画やプレイリストのURLを渡すだけで、プログラムが自動的にリンクの種類を判別し、それに応じて質問します。

最初の質問は `ダウンロードしたいファイルの種類は何ですか？` です

選択した答えによって、後で選択できるダウンロード形式が決まります：例えば、音声のみのファイルをダウンロードしたいと答えた場合、動画を含む形式は非表示になります。このREADMEでは、`動画`のダウンロードに関する記述は、音声のみのダウンロードにも適用されます

2番目の質問 `動画に適用する品質や形式はどれですか？` では、特定の形式、品質、ファイルサイズなどを選択できます。

利用可能な回答の意味：

- `最良の品質` は、yt-dlpに自動的に `最良` の品質を選択するように指示します。詳細については、`yt-dlp` の[Wiki](https://github.com/yt-dlp/yt-dlp#format-selection)を参照してください

- `最小のファイルサイズ` は、最小のファイルサイズになる形式を使用します

- `動画を再エンコードする形式を選択` は、ffmpegがインストールされている場合のみ利用可能です：動画をダウンロードした後、選択したファイル形式に変換できます

- `動画をダウンロードする形式を選択` はffmpegを必要としません：変換することなくYouTubeから直接ダウンロード可能な形式のリストを表示しますが、選択肢は限られています


`blob-dl` は他の質問もしますが、それらは自明です

# 機能

### 形式変換
`blob-dl` は、大きな楽曲プレイリストを直接音声ファイルとしてダウンロードするように設計されています。音声ファイル、通常の動画ファイル、または動画のみのファイルのどれをダウンロードするかを選択するのは非常に簡単です

### プレイリストのダウンロード
`blob-dl` を使用すると、プレイリスト全体を一度にダウンロードでき、すべての動画に適用する単一のファイル形式を選択することもできます

### エラー追跡

ダウンロード中、`blob-dl` はyt-dlpによってスローされたエラーを追跡し、最後にレポートします。再試行することで解決できるエラーは、簡単に再ダウンロードできます

## 設定ファイル
同じ設定を使用して頻繁に動画をダウンロードしていて、常に同じ質問に答えることにうんざりしている場合は、設定ファイルを使用する時です！

設定ファイルにより、`blob-dl` があなたの希望を既に知っているため、多くの質問を避けることができます。

### 設定ファイルの作成方法
新しい設定ファイルの作成は簡単なプロセスです：`-g` フラグを使用するだけで、`blob-dl` がデフォルトの場所にあなたの回答を含む設定ファイルを生成します
```
$ blob-dl -g "youtube url"
```
`-g` を複数回使用すると、古い設定ファイルが上書きされることに注意してください：デフォルトの場所には一度に1つだけ存在します



####  デフォルトの設定ファイルの場所

|    OS     |                                設定パス                                |
|:---------:|:-------------------------------------------------------------------------:|
| **Linux** | `~/.config/blob-dl/config.json` または <br/> `$XDG_CONFIG_HOME/blob-dl/config.json`|
| **macOS** |            `~/Library/Application Support/blob-dl/config.json`            |
| **Windows** |                      `%APPDATA%\blob-dl\config.json`                      |

#### 注意事項：
- **Linux** では、`$XDG_CONFIG_HOME` が設定されていない場合、デフォルトは `~/.config` です。
- **macOS** では、`~/Library` は通常非表示になっています。Finderで `Cmd + Shift + .` を使用して隠しフォルダを表示できます。
- **Windows** では、`%APPDATA%` は通常 `C:\Users\YourName\AppData\Roaming` に解決されます。

### 使用方法

`-c` または `--use-config` を使用して、`blob-dl` のデフォルトの場所にある設定ファイルを使用します
```
$ blob-dl -c "youtube url"
```
設定ファイルを別の場所に移動した場合は、`-l "filepath"` を使用する必要があります
```
$ blob-dl -l "/Users/YourName/Desktop/config.json" "youtube url"
```

### 設定ファイルの編集方法
`blob-dl` の設定ファイルは次のようになります：


ファイル名: `config.json`
```
{
    "url": null,
    "output_path": "/Users/YourName/Desktop",
    "include_indexes": false,
    "chosen_format": "BestQuality",
    "media_selected": "FullVideo",
    "download_target": "YtPlaylist"
}
```
これらの各フィールドは null に設定できます。その場合、`blob-dl` はあなたが空白にした部分に関連する質問をします

`url` は `blob-dl` によって無視されるため、常に null のままにできます

`output_path` は、ダウンロードするファイルが保存される場所で、パスを指定する必要があります

`include_indexes` はブール値です：プレイリストをダウンロードする際、ファイル名の一部としてプレイリスト内の動画の位置を含めることができます（例：1_firstvideo、2_secondvideo、...）

`chosen_format` は、ファイルの形式です。いくつかのオプションがあります：

|        オプション         |                                                                                                         機能                                                                                                         |
|:---------------------:|:----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------:|
|    **BestQuality**    |                                                                            `blob-dl` は利用可能な最高品質で動画/音声をダウンロードします                                                                            |
|   **SmallestSize**    |                                                                          `blob-dl` は利用可能な最小ファイルサイズで動画/音声をダウンロードします                                                                         |
| **ConvertTo(format)** |                                                            動画/音声をダウンロードした後、`blob-dl` はffmpegを使用して選択した形式に変換します                                                             |
| **UniqueFormat(id)**  | これはエンドユーザーが編集することを想定していません：各可能な形式には数値IDがあります。問題は、特定の形式が複数の動画で利用可能である可能性が低いため、設定ファイルには適していないことです |

ConvertTo(format) の使用構文：
```
  "chosen_format": {
    "ConvertTo": "mp4"
  },
```
この機能は、ffmpegがサポートするすべての形式をサポートしています：`mp4, mkv, mov, avi, flv, gif, webm, aac, aiff, alac, flac, m4a, mka, mp3, ogg, opus, vorbis, wav`

`media_selection` は、通常の動画、音声のみ、または動画のみをダウンロードするかどうかを指します。
文字列を想定し、利用可能なオプションは：`FullVideo` `AudioOnly` `VideoOnly` 

`download_target` は、単一の動画をダウンロードするか、プレイリスト全体をダウンロードするかです。
文字列を想定し、オプションは `YtPlaylist`（通常の動画をダウンロードする場合でも、ほとんどの場合に使用されるべき）と `YtVideo(index)`（プレイリストから単一の動画をダウンロードする場合のみ必要で、プレイリスト内のインデックスを指定する必要があります）です

# Q&A
### 誰のためのものですか？
このプログラムは、`yt-dlp` の構文を覚えることなく、YouTubeからコンテンツをダウンロードしたい人のためのものです。`blob-dl` は、平均的なユーザーが必要とするすべてのことを、より簡単に実行できます

高度なニーズを持つ `yt-dlp` パワーユーザーは、このプログラムを有用だと感じないでしょう。

### なぜこれを作ったのか？
YouTubeから動画をダウンロードする必要があったことはありますか？
プロセスは非常に面倒な場合があります。怪しげな Web サイトからのポップアップを閉じるのに時間を費やすか、[`yt-dlp`](https://github.com/yt-dlp/yt-dlp) のドキュメントを閲覧する必要があるためです。

音楽動画をダウンロードして音声に変換するのに何時間も費やすことにうんざりしていたので、すべてをより簡単にするためにこのプログラムを書きました

## 注記
このロゴは [@Primer](https://www.youtube.com/c/PrimerLearning) の[blob plushie](https://store.dftba.com/collections/primer/products/primer-blob-plushie) にインスパイアされました
