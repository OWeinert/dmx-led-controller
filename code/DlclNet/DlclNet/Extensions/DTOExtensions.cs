using DlclNet.Models;
using DlclRpc;

namespace DlclNet.Extensions;

public static class DTOExtensions
{
    public static Animation ToAnimation(this AnimationDTO dto)
    {
        var frames = dto.Frames.ToList()
            .ConvertAll(f => f.ToFrame())
            .Cast<IFrame>()
            .ToList();
        var anim = new Animation
        {
            Layer = dto.Layer,
            Frames = frames
        };
        return anim;
    }

    public static SingleFrame ToFrame(this FrameDTO dto)
    {
        var pixels = dto.Pixels.ToList().ConvertAll(p => p.ToPixel());
        var frame = new SingleFrame
        {
            Pixels = pixels
        };
        return frame;
    }

    public static Pixel ToPixel(this PixelDTO dto)
    {
        var pixel = new Pixel
        {
            Color = new Color
            {
                R = dto.R,
                G = dto.G,
                B = dto.B
            },
            Position = new Position
            {
                X = dto.X,
                Y = dto.Y
            }
        };
        return pixel;
    }
}