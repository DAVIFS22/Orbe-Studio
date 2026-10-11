"""Local U2Net background segmentation adapter and real inference self-test.
Normal mode: base64 PNG stdin -> base64 PNG stdout. Self-test: --self-test.
"""
import base64
import io
import json
import logging
import sys
import time

logging.basicConfig(stream=sys.stderr, level=logging.ERROR)

def run_inference(raw: bytes) -> bytes:
    from rembg import remove, new_session
    session = new_session("u2net")
    return remove(raw, session=session, force_return_bytes=True)

def self_test() -> int:
    from PIL import Image, ImageDraw
    import rembg
    import onnxruntime
    image = Image.new("RGB", (256, 256), (235, 238, 242))
    draw = ImageDraw.Draw(image)
    draw.ellipse((72, 25, 184, 137), fill=(194, 80, 60))
    draw.rounded_rectangle((52, 115, 204, 240), radius=28, fill=(45, 93, 155))
    src = io.BytesIO(); image.save(src, format="PNG")
    start = time.perf_counter()
    result = run_inference(src.getvalue())
    elapsed = time.perf_counter() - start
    with Image.open(io.BytesIO(result)) as out:
        out.load()
        if out.format != "PNG":
            raise RuntimeError(f"Formato inesperado na saída: {out.format}")
        if out.size != image.size:
            raise RuntimeError(f"Dimensões alteradas: entrada {image.size}, saída {out.size}")
        if "A" not in out.getbands():
            raise RuntimeError("A saída não contém canal alfa; a máscara não foi produzida.")
        alpha = out.getchannel("A")
        lo, hi = alpha.getextrema()
        report = {
            "status": "PASS", "model": "u2net", "inference_executed": True,
            "rembg_version": getattr(rembg, "__version__", "unknown"),
            "onnxruntime_version": getattr(onnxruntime, "__version__", "unknown"),
            "providers_available": onnxruntime.get_available_providers(),
            "input_size": list(image.size), "output_size": list(out.size),
            "alpha_range": [lo, hi], "elapsed_seconds": round(elapsed, 2),
            "note": "Teste confirma execução e saída do modelo; a qualidade visual deve ser avaliada com uma foto real."
        }
        print(json.dumps(report, ensure_ascii=False, indent=2))
    return 0

def main() -> int:
    if "--self-test" in sys.argv[1:]:
        try:
            return self_test()
        except Exception as exc:
            print(json.dumps({"status":"FAIL", "inference_executed":False, "error":f"{type(exc).__name__}: {exc}"}, ensure_ascii=False, indent=2))
            return 2
    payload = sys.stdin.buffer.read().strip()
    if not payload:
        raise ValueError("Nenhuma imagem recebida")
    raw = base64.b64decode(payload, validate=True)
    result = run_inference(raw)
    sys.stdout.write(base64.b64encode(result).decode("ascii"))
    return 0

if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as exc:
        print(f"{type(exc).__name__}: {exc}", file=sys.stderr)
        raise SystemExit(2)
