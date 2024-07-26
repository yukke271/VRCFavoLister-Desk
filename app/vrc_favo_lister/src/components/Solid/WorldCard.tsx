import type { AppFavoriteWorldCard } from "~/src/types/app_types";
import WorldCardItem from "~/src/components/Solid/WorldCardItem";

type AppFavoriteWorldCardProps = {

  items: AppFavoriteWorldCard[];

};

export default function WorldCard(props: AppFavoriteWorldCardProps) {
  return (
    <div class={`grid grid-cols-3 gap-4`}>
      {props.items.map((item) => (
        <WorldCardItem item={item} />
      ))}
    </div>
  );
}