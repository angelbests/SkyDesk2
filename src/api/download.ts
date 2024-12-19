import { exists, writeFile } from "@tauri-apps/plugin-fs";
import { fetch } from "@tauri-apps/plugin-http";

// 单fetch下载
export const downloadload = (path: string, src: string): Promise<boolean> => {
  return new Promise(async (resolve) => {
    try {
      let f = await fetch(src);
      let reader = f.body?.getReader();
      while (true) {
        let res = await reader?.read();
        if (!res?.done) {
          if (res?.value) {
            await writeFile(path, res.value);
          }
          break;
        }
      }
      resolve(true);
    } catch (error) {
      resolve(true);
    }
  });
};

export class multidownload {
  private arr: {
    name: string;
    src: string;
    error?: any;
  }[] = [];
  private thread: number = 1;
  private bools: {
    status: boolean;
    index: number;
  }[] = [];
  private index: number = 0;
  private timer: boolean = true;
  public dir: string;

  public constructor(
    downloadlist: {
      name: string;
      src: string;
      error?: any;
    }[],
    dir: string,
    thread?: number
  ) {
    this.arr = downloadlist;
    this.dir = dir;
    if (thread) {
      this.thread = thread;
    }
    for (let i = 0; i < this.thread; i++) {
      this.bools.push({
        status: true,
        index: 0,
      });
    }
  }

  public ondownload:
    | ((e: {
        item: {
          name: string;
          src: string;
        };
        message: string;
        index: number;
      }) => void)
    | undefined;

  public stopdownload() {
    this.timer = false;
  }

  public cancaldownload() {
    this.timer = false;
  }

  public startdownload() {
    this.timer = true;
    this.downloadcontrol();
  }

  private async downloadcontrol() {
    while (this.timer) {
      console.log("下载中");
      for (let i = 0; i < this.thread; i++) {
        // 判断是否完成
        if (this.index >= this.arr.length) {
          this.timer = false;
          this.index = this.arr.length;
          continue;
        }
        // 判断每个fetch是否完成
        if (this.bools[i].status) {
          console.log("下载到：", this.index);
          this.bools[i].status = false;
          let path = this.dir + "\\" + this.arr[this.index].name;
          // 判断文件是否存在
          if (!(await exists(path))) {
            if (this.ondownload != undefined) {
              this.ondownload({
                item: this.arr[this.index],
                index: this.index,
                message: "下载中",
              });
            }
            this.download(this.arr[this.index].src, path, i, this.index);
            this.bools[i].index = this.index;
            this.index++;
          } else {
            this.arr[this.index].error = JSON.stringify({
              message: "文件已存在！",
            });
            if (this.ondownload != undefined) {
              this.ondownload({
                item: this.arr[this.index],
                index: this.index,
                message: "已存在",
              });
            }
            this.bools[i].status = true;
            this.bools[i].index = this.index;
            this.index++;
          }
        }
      }
      await this.sleep(10);
    }
  }

  private async download(src: string, path: string, i: number, index: number) {
    try {
      let f: Response | undefined = await fetch(src, {
        connectTimeout: 3000,
      });
      let reader = f.body?.getReader();
      while (true) {
        await this.sleep(10);
        let res = await reader?.read();
        if (!res?.done) {
          if (res?.value) {
            await writeFile(path, res.value);
          }
          break;
        }
      }
      if (this.ondownload != undefined) {
        this.ondownload({
          item: this.arr[index],
          index: index,
          message: "已完成",
        });
      }
      this.bools[i].status = true;
    } catch (error) {
      if (this.ondownload != undefined) {
        this.ondownload({
          item: this.arr[index],
          index: index,
          message: "错误",
        });
      }
      this.bools[i].status = true;
    }
  }

  private sleep(n: number): Promise<boolean> {
    return new Promise((resolve) => {
      setTimeout(() => {
        resolve(true);
      }, n);
    });
  }
}
