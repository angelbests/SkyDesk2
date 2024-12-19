import { fetch } from "@tauri-apps/plugin-http";

export const getwallpaper = async function (select: {
  categories: string;
  sorting: string;
  order: string;
  ratio: string;
  resolutions: string;
  keyword: string;
  page: number;
}) {
  // let href = "https://wallhaven.cc/api/v1/search?purity=100";
  // categories  sorting  order  resolutions  ratios
  let href = "https://www.skymiao.info/api/v1/search?purity=100";
  if (select.categories != "") {
    href = href + "&categories=" + select.categories;
  }
  if (select.sorting != "") {
    href = href + "&sorting=" + select.sorting;
  }
  if (select.order != "") {
    href = href + "&order=" + select.order;
  }
  if (select.ratio != "") {
    href = href + "&ratios=" + select.ratio;
  }
  if (select.resolutions != "") {
    href = href + "&resolutions=" + select.resolutions;
  }
  if (select.keyword != "") {
    href = href + "&q=" + select.keyword;
  }
  if (select.page != 0) {
    href = href + "&page=" + select.page;
  }
  let response;
  try {
    response = await fetch(href, {
      method: "GET",
    });
  } catch (error) {
    console.log(error);
    return [];
  }
  if (response.ok) {
    let res = await response.json();
    console.log(res);
    if (res.data != undefined) {
      for (let i = 0; i < res.data.length; i++) {
        res.data[i].path = res.data[i].path.replace(
          "wallhaven.cc",
          "skymiao.info"
        );
        res.data[i].url = res.data[i].url.replace(
          "wallhaven.cc",
          "skymiao.info"
        );
        res.data[i].thumbs.large = res.data[i].thumbs.large.replace(
          "wallhaven.cc",
          "skymiao.info"
        );
        res.data[i].thumbs.original = res.data[i].thumbs.original.replace(
          "wallhaven.cc",
          "skymiao.info"
        );
        res.data[i].thumbs.small = res.data[i].thumbs.small.replace(
          "wallhaven.cc",
          "skymiao.info"
        );
      }
    } else {
      res.data = [];
    }
    console.log(res);

    return res;
  } else {
    return false;
  }
};

export const httpload = (s: {
  page: number;
  categories: any;
  sorting: any;
  order: any;
  ratio: any;
  resolutions: any;
  keyword: any;
}): Promise<{ data: any[]; res: any }> => {
  return new Promise(async (resolve) => {
    let res = await getwallpaper(s);
    resolve(res);
  });
};
