import { Metadata, MetadataMap, MetadataMapEntry } from './player.schema';

const parseMetadataEntry = (entry: MetadataMapEntry): string => {
  if (!entry || !entry.signature) return '';

  switch (entry.signature) {
    case 'i':
    case 'd':
    case 't':
      return `${entry.value}`;

    case 's':
      return entry.value;
    case 'as':
      return entry.value.join(' ');
  }
};

export const parseMetadata = (metadata: MetadataMap): Partial<Metadata> => {
  return {
    title: parseMetadataEntry(metadata['xesam:title']),
    artist: parseMetadataEntry(metadata['xesam:artist']),
    album: parseMetadataEntry(metadata['xesam:album']),
    cover: parseMetadataEntry(metadata['mpris:artUrl']),
    length: Number.parseFloat(parseMetadataEntry(metadata['mpris:length'])),
  };
};
