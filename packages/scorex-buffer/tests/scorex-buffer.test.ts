import { ScorexWriter, ScorexReader } from "../";

describe("ScorexBuffer", () => {
  it("should write and read string values", () => {
    const w = new ScorexWriter();

    w.putString("hello world");

    const r = new ScorexReader(w.buffer);

    expect(r.getString()).toBe("hello world");
  });

  it("should write and read complex", () => {
    const w = new ScorexWriter();

    w.putUint64(50012144444n);
    w.putInt8(1);
    w.putString("ErGonde");
    w.putUint16(255);

    const r = new ScorexReader(w.buffer);

    expect(r.getUint64()).toBe(50012144444n);
    expect(r.getInt8()).toBe(1);
    expect(r.getString()).toBe("ErGonde");
    expect(r.getUint16()).toBe(255);
  });
});
