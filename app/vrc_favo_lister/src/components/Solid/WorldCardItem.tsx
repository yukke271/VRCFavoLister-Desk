import type { AppFavoriteWorldCard } from "~/src/types/app_types";
import "./css/WorldCardItem.css";

type WorldCardItemProps = {

  item: AppFavoriteWorldCard;

};

export default function WorldCardItem(props: WorldCardItemProps) {

  let isPC = null;
  let isQuest = null;
  if ( props.item.platform.includes("PCOnly") ) {
    isPC = true;
  } else if ( props.item.platform.includes("QuestOnly") ) {
    isQuest = true;
  } else if ( props.item.platform.includes("CrossPlatform") ) {
    isPC = true;
    isQuest = true;
  } 
  
  // image_urlは"file_xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx/1"である場合、
  // thumbnailImageUrlは"https://api.vrchat.cloud/api/1/image/file_xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx/1/256"になり、
  // サムネイルじゃない画像は"https://api.vrchat.cloud/api/1/file/file_xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx/1/file"になる。
  let thumbnailImageUrl = "https://api.vrchat.cloud/api/1/image/" + props.item.imageId + "/256";
  
  return (
    <a
      href={`https://vrchat.com/home/world/${props.item.id}`}
      target="_blank"
      class="relative box-border m-1 "
    >
      <img 
        src={thumbnailImageUrl} 
        alt={props.item.name}
        class="w-full"
      />
      <div class="absolute w-full">
        {/* TODO:幅の指定を、相対的に指定したい。 */}
        <div class={`absolute inset-x-0 bottom-0 w-full `}>
          <p class="title truncate">{props.item.name}</p>
        </div>
        <div class={`pc absolute right-6 bottom-28 ${isPC ? '' : 'grayout'}`}>
          <span>PC</span>
        </div>
        <div class={`quest absolute right-2 bottom-28 ${isQuest ? '' : 'grayout'}`}>
          <span>Quest</span>
        </div>
      </div>
    </a>
  );
}
