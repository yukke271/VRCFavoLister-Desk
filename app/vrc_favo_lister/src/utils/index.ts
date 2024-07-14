export const isMobile = () => {
  // サイトにアクセスしている端末がモバイル端末かどうかを画面の幅で判定する
  if (
    typeof window.matchMedia === "function" &&
    window.matchMedia("(max-device-width: 640px)").matches
  ) {
    return true;
  } else {
    return false;
  }
};

export const devLog = (text: string) => {
  const envType = import.meta.env.MODE;
  if (envType !== "development") return;
  console.log(text);
};

export const downloadJSON = (List: any[]) => {
  // 重複を排除する
  const uniqList = List.filter((element, index, self) => {
    return (
      self.findIndex((e) => {
        return e.id === element.id;
      }) === index
    );
  });
  // JSON形式にする
  const json = JSON.stringify(uniqList, null, 2);
  // ダウンロードさせる
  const blob = new Blob([json], { type: "application/json" });
  const link = document.createElement("a");
  link.href = URL.createObjectURL(blob);
  link.download = "VRCFavoLister.json";
  link.click();
};
