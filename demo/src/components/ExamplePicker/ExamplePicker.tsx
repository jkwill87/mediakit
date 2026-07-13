import './ExamplePicker.css'

const EXAMPLES = [
  'aladdin.2019.avi',
  'Ninja Turtles (1990).mkv',
  'game.of.thrones.01x05-eztv.mp4',
  'lost s01e01-02.mp4',
  'archer.2009.s10e07.webrip.x264-lucidtv.mp4',
  'Planet Earth II S01E06 - Cities (2016) (2160p).mp4',
  'O.J. - Made in America S01EP03 (2016) (1080p).mp4',
  'Star.Trek.Strange.New.Worlds.S02E02.Ad.Astra.per.Aspera.2160p.AMZN.WEB-DL.DDP5.1.H.265-NTb[TGx].mkv',
  'The.Amazing.Spider-Man.2.2014.2160p.UHD.BluRay.DTS-HD.MA.5.1.HEVC.Main10-ARCHiViST.mkv',
  'Venom.Let.There.Be.Carnage.2021.2160p.WEB-DL.DDP5.1.H.265.Main10-FLAME.mkv',
  'The.Little.Mermaid.2023.en.forced.srt',
  'The.Instigators.2024.en.2.commentary.vtt',
  'Silo.S01E02.pt-BR.sdh.ass',
]

interface ExamplePickerProps {
  onSelect: (filename: string) => void
}

export default function ExamplePicker({ onSelect }: ExamplePickerProps) {
  return (
    <div className='examples'>
      <span className='examples-label'>Examples</span>
      <div className='examples-list'>
        {EXAMPLES.map((name) => (
          <button
            key={name}
            className='example-chip'
            onClick={() => onSelect(name)}
            type='button'
          >
            {name}
          </button>
        ))}
      </div>
    </div>
  )
}
