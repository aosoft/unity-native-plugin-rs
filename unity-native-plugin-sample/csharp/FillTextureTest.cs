using System;
using System.Runtime.InteropServices;
using UnityEngine;
using UnityEngine.Rendering;

[RequireComponent(typeof(Renderer))]
public class FillTextureTest : MonoBehaviour
{
    private const string DllName = "unity_native_plugin_sample";

    [DllImport(DllName)]
    private static extern IntPtr GetFillTextureCallback();

    [StructLayout(LayoutKind.Sequential)]
    private struct FillTextureParams
    {
        public IntPtr Texture;
        public float X;
        public float Y;
        public float Z;
        public float W;
    }

    [SerializeField] private int textureSize = 256;
    [SerializeField] private Color color = Color.red;
    [SerializeField] private bool animate = true;
    [SerializeField] private float animationSpeed = 1.0f;
    [SerializeField] private Camera targetCamera;

    private RenderTexture _renderTexture;
    private Material _material;
    private CommandBuffer _commandBuffer;
    private IntPtr _paramsPtr = IntPtr.Zero;
    private IntPtr _callback = IntPtr.Zero;

    private void Start()
    {
        _renderTexture = new RenderTexture(textureSize, textureSize, 0, RenderTextureFormat.ARGB32)
        {
            name = "FillTextureTarget",
            enableRandomWrite = false,
        };
        _renderTexture.Create();

        var r = GetComponent<Renderer>();
        _material = r.material;
        _material.SetColor("_BaseColor", Color.white);
        _material.SetColor("_Color", Color.white);
        _material.mainTexture = _renderTexture;

        _commandBuffer = new CommandBuffer { name = "FillTexture" };
        _paramsPtr = Marshal.AllocHGlobal(Marshal.SizeOf<FillTextureParams>());
        _callback = GetFillTextureCallback();
    }

    private void OnEnable()
    {
        RenderPipelineManager.beginCameraRendering += OnBeginCameraRendering;
    }

    private void OnDisable()
    {
        RenderPipelineManager.beginCameraRendering -= OnBeginCameraRendering;
    }

    private void OnBeginCameraRendering(ScriptableRenderContext context, Camera cam)
    {
        if (_renderTexture == null || !_renderTexture.IsCreated())
        {
            return;
        }
        if (_callback == IntPtr.Zero || _paramsPtr == IntPtr.Zero || _commandBuffer == null)
        {
            return;
        }

        Camera filterCam = targetCamera != null ? targetCamera : Camera.main;
        if (filterCam != null && cam != filterCam)
        {
            return;
        }

        Color c = color;
        if (animate)
        {
            float t = Mathf.Repeat(Time.time * animationSpeed, 1.0f);
            c = Color.HSVToRGB(t, 1.0f, 1.0f);
            c.a = color.a;
        }

        IntPtr nativePtr = _renderTexture.GetNativeTexturePtr();
        if (nativePtr == IntPtr.Zero)
        {
            return;
        }

        var p = new FillTextureParams
        {
            Texture = nativePtr,
            X = c.r,
            Y = c.g,
            Z = c.b,
            W = c.a,
        };
        Marshal.StructureToPtr(p, _paramsPtr, false);

        _commandBuffer.Clear();
        _commandBuffer.IssuePluginEventAndData(_callback, 0, _paramsPtr);
        context.ExecuteCommandBuffer(_commandBuffer);
    }

    private void OnDestroy()
    {
        if (_commandBuffer != null)
        {
            _commandBuffer.Release();
            _commandBuffer = null;
        }

        if (_paramsPtr != IntPtr.Zero)
        {
            Marshal.FreeHGlobal(_paramsPtr);
            _paramsPtr = IntPtr.Zero;
        }

        if (_renderTexture != null)
        {
            _renderTexture.Release();
            UnityEngine.Object.Destroy(_renderTexture);
            _renderTexture = null;
        }
    }
}
